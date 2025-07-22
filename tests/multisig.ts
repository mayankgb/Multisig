import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Multisig } from "../target/types/multisig";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { createMint, getAccount, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID } from "@solana/spl-token";
describe("multisig", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);


  const program = anchor.workspace.multisig as Program<Multisig>;
  const LAMPORTS_PER_SOL = 1000_000_000;
  const USDC_DECIMALS = 1000_000;

  let usdcMint: anchor.web3.PublicKey;
  let vaultAta: anchor.web3.PublicKey;
  let withdrawlAta: anchor.web3.PublicKey;



  const wallet = provider.wallet as NodeWallet


  const signer1 = anchor.web3.Keypair.generate();
  const signer2 = anchor.web3.Keypair.generate();
  const signer3 = anchor.web3.Keypair.generate();
  const withdrawalAcc = anchor.web3.Keypair.generate()


  const fakesigner = anchor.web3.Keypair.generate()

  const randomSeed = new anchor.BN(42)

  const [vaultPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("vault"),
      signer1.publicKey.toBuffer(),
      randomSeed.toArrayLike(Buffer, "le", 8)
    ],
    program.programId
  )

  const vaultDataAccount = anchor.web3.Keypair.generate();

  it("airdrop some sol into the signers wallet", async () => {
    const signer1Tx = await provider.connection.requestAirdrop(
      signer1.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    const signer2Tx = await provider.connection.requestAirdrop(
      signer2.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    const signer3Tx = await provider.connection.requestAirdrop(
      signer3.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    const fakeSignerTx = await provider.connection.requestAirdrop(
      fakesigner.publicKey,
      3 * LAMPORTS_PER_SOL
    )
    const withdrawalAccTx = await provider.connection.requestAirdrop(
      withdrawalAcc.publicKey,
      3 * LAMPORTS_PER_SOL
    );

    await provider.connection.confirmTransaction(withdrawalAccTx)
    await provider.connection.confirmTransaction(fakeSignerTx)
    await provider.connection.confirmTransaction(signer1Tx)
    await provider.connection.confirmTransaction(signer2Tx)
    await provider.connection.confirmTransaction(signer3Tx)
  })

  it("initialise a mint and transfer some token into the users wallet", async () => {
    usdcMint = await createMint(
      provider.connection,
      wallet.payer,
      wallet.publicKey,
      null,
      6,
    )

    vaultAta = (await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      usdcMint,
      vaultPda,
      true,
    )).address

    const res = await mintTo(
      provider.connection,
      wallet.payer,
      usdcMint,
      vaultAta,
      wallet.publicKey,
      100 * USDC_DECIMALS
    )

    withdrawlAta = getAssociatedTokenAddressSync(
      usdcMint,
      withdrawalAcc.publicKey,
      false,
    )

    // await provider.connection.confirmTransaction(res);

  })



  it("Is initialized!", async () => {
    // Add your test here.
    const threshold = 2;
    const tx = await program.methods.initializeMultisig(randomSeed, new anchor.BN(threshold))
      .accountsPartial({
        signer: signer1.publicKey,
        vault: vaultPda,
      })
      .remainingAccounts([{
        isSigner: true,
        pubkey: signer1.publicKey,
        isWritable: false,
      },
      {
        isSigner: false,
        pubkey: signer2.publicKey,
        isWritable: false,
      },
      {
        isSigner: false,
        pubkey: signer3.publicKey,
        isWritable: false,
      }
      ])
      .signers([signer1])
      .rpc()
      ;
    console.log("Your transaction signature", tx);
  });

  it("initialise transaction", async () => {

    const amount = 2 * USDC_DECIMALS

    console.log("signer 1 public key", signer1.publicKey.toBase58())
    console.log("signer 2 public key", signer2.publicKey.toBase58())
    console.log("vault pda public key", vaultPda.toBase58())

    const tx = await program.methods.initialiseTransaction(randomSeed, new anchor.BN(amount))
      .accountsPartial({
        signer: signer2.publicKey,
        transferTo: withdrawalAcc.publicKey,
        vault: vaultPda,
        creator: signer1.publicKey,
        vaultTx: vaultDataAccount.publicKey,
        mint: usdcMint,
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .transaction()

    const { blockhash } = await provider.connection.getLatestBlockhash()
    tx.recentBlockhash = blockhash;
    tx.feePayer = signer2.publicKey;
    const res = await provider.connection.sendTransaction(tx, [signer2, vaultDataAccount])
    const signature = await provider.connection.confirmTransaction(res)
    console.log("Your transaction signature", signature);
  })

  it("approve transaction by fake signer ", async () => {

    //This test must fail

    const tx = await program.methods.approveTx(randomSeed)
      .accountsPartial({
        creator: signer1.publicKey,
        vault: vaultPda,
        vaultTx: vaultDataAccount.publicKey,
        signer: fakesigner.publicKey,
      })
      .signers([fakesigner])
      .rpc();

    console.log("your transaction signature", tx);

  })

  it("signers want to withdraw the transaction without all signature", async () => {

    //this test must fail because enough signature is not present

    const tx = await program.methods.withdrawTx(randomSeed)
      .accountsPartial({
        signer: signer2.publicKey,
        withdrawAcc: withdrawalAcc.publicKey,
        withdrawAta: withdrawlAta,
        creatorAcc: signer1.publicKey,
        vault: vaultPda,
        vaultTx: vaultDataAccount.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        vaultAta: vaultAta,
        mint: usdcMint
      })
      .signers([signer2])
      .rpc()

    console.log("Your transaction signature is ", tx);
  })

  it("approve transaction by authorised signer ", async () => {

    const tx = await program.methods.approveTx(randomSeed)
      .accountsPartial({
        creator: signer1.publicKey,
        vault: vaultPda,
        vaultTx: vaultDataAccount.publicKey,
        signer: signer1.publicKey,
      })
      .signers([signer1])
      .rpc();

    console.log("your transaction signature", tx);

  })


  it("signers want to withdraw the transaction with all signature present ", async () => {

    //this test must fail because enough signature is not present

    const tx = await program.methods.withdrawTx(randomSeed)
      .accountsPartial({
        signer: signer2.publicKey,
        withdrawAcc: withdrawalAcc.publicKey,
        withdrawAta: withdrawlAta,
        creatorAcc: signer1.publicKey,
        vault: vaultPda,
        vaultTx: vaultDataAccount.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        vaultAta: vaultAta,
        mint: usdcMint
      })
      .signers([signer2])
      .rpc()

    console.log("Your transaction signature is ", tx);
    const getAccountWithdrawalAtaInfo = await getAccount(provider.connection, withdrawlAta) 

    console.log("this is the balance present in the withdrawal ata",Number(getAccountWithdrawalAtaInfo.amount)/USDC_DECIMALS);
  })


  it("approve the transaction which is already executed by authorised signer ", async () => {

    //this test must fail 

    const tx = await program.methods.approveTx(randomSeed)
      .accountsPartial({
        creator: signer1.publicKey,
        vault: vaultPda,
        vaultTx: vaultDataAccount.publicKey,
        signer: signer1.publicKey,
      })
      .signers([signer1])
      .rpc();

    console.log("your transaction signature", tx);

  })
  

});
