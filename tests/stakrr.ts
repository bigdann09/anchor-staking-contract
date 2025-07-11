import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Stakrr } from "../target/types/stakrr";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createAssociatedTokenAccount, createMint, getAssociatedTokenAddressSync, getMint, mintTo, TOKEN_PROGRAM_ID } from "@solana/spl-token"
import { assert } from "chai";

describe("stakrr", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.stakrr as Program<Stakrr>;
  const connection = provider.connection;

  // let lstMint: anchor.web3.PublicKey;
  let reward_mint: anchor.web3.PublicKey;
  let staking_mint: anchor.web3.PublicKey;

  const bob = anchor.web3.Keypair.generate();
  let bob_reward_token_account: anchor.web3.PublicKey;
  let bob_staking_token_account: anchor.web3.PublicKey;

  const alice = anchor.web3.Keypair.generate();
  let alice_reward_token_account: anchor.web3.PublicKey;
  let alice_staking_token_account: anchor.web3.PublicKey;

  let admin_reward_token_account: anchor.web3.PublicKey;

  // pdas
  let pool_pda: anchor.web3.PublicKey;
  let lst_mint: anchor.web3.PublicKey;
  let reward_vault_pda: anchor.web3.PublicKey;
  let staked_vault_pda: anchor.web3.PublicKey;

  before(async () => {
    // airdrop wallets
    const wallets = [bob, alice, provider.wallet]
    wallets.map(wallet => {
      connection.requestAirdrop(wallet.publicKey, 1000_000_000_000)
    })

    // create staking token mint
    staking_mint = await createMint(
      connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      9,
    );

    // create reward token mint
    reward_mint = await createMint(
      connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      6,
    );

    // create user's token account
    admin_reward_token_account = await createAssociatedTokenAccount(
      connection,
      provider.wallet.payer,
      reward_mint,
      provider.wallet.publicKey
    )

    bob_staking_token_account = await createAssociatedTokenAccount(
      connection,
      bob,
      staking_mint,
      bob.publicKey
    )

    bob_reward_token_account = await createAssociatedTokenAccount(
      connection,
      bob,
      reward_mint,
      bob.publicKey
    )

    alice_staking_token_account = await createAssociatedTokenAccount(
      connection,
      alice,
      staking_mint,
      alice.publicKey
    )

    alice_reward_token_account = await createAssociatedTokenAccount(
      connection,
      alice,
      reward_mint,
      alice.publicKey
    )

    // mint tokens to token accounts
    await mintTo(
      connection,
      provider.wallet.payer,
      reward_mint,
      admin_reward_token_account,
      provider.wallet.publicKey,
      7000 * Math.pow(10, 6)
    )

    await mintTo(
      connection,
      provider.wallet.payer,
      staking_mint,
      bob_staking_token_account,
      provider.wallet.publicKey,
      50000 * Math.pow(10, 9)
    )

    await mintTo(
      connection,
      provider.wallet.payer,
      staking_mint,
      alice_staking_token_account,
      provider.wallet.publicKey,
      50000 * Math.pow(10, 9)
    )

    // pdas
    pool_pda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pool")],
      program.programId
    )[0]

    reward_vault_pda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("reward_vault"), reward_mint.toBuffer()],
      program.programId
    )[0]

    staked_vault_pda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("staked_vault"), staking_mint.toBuffer()],
      program.programId
    )[0]

    lst_mint = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("lst_mint"), staking_mint.toBuffer()],
      program.programId
    )[0]
  })

  function get_lst_token_account(wallet: anchor.web3.PublicKey) {
    const wallet_ata = getAssociatedTokenAddressSync(
      lst_mint,
      wallet,
      false,
      TOKEN_PROGRAM_ID
    )
    return wallet_ata
  }

  function get_user_stake_info_pda(wallet: anchor.web3.PublicKey) {
    const user_pda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), wallet.toBuffer(), pool_pda.toBuffer()],
      program.programId
    )[0]
    return user_pda
  }

  it("initialize staking program", async () => {
    // Add your test here.
    const initializeTx = await program.methods.initialize(0.00001)
      .accountsPartial({
        pool: pool_pda,
        rewardTokenMint: reward_mint,
        stakedTokenMint: staking_mint,
        tokenProgram: TOKEN_PROGRAM_ID,
        signer: provider.wallet.publicKey
      }).instruction();

    const latestblockhash = await connection.getLatestBlockhash();
    const tx = new anchor.web3.Transaction({
      feePayer: provider.wallet.publicKey,
      blockhash: latestblockhash.blockhash,
      lastValidBlockHeight: latestblockhash.lastValidBlockHeight
    }).add(initializeTx)

    await anchor.web3.sendAndConfirmTransaction(
      connection,
      tx,
      [provider.wallet.payer],
      {skipPreflight: true}
    );

    // fetch pool account
    const pool_account = await program.account.pool.fetch(pool_pda)
    assert.equal(pool_account.rewardTokenMint.toBase58(), reward_mint.toBase58())
    assert.equal(pool_account.stakedTokenMint.toBase58(), staking_mint.toBase58())
    assert.equal(pool_account.rewardRatePerSecond, 0.00001)
  });

  it("fund reward vault", async () => {
    const fundIx = await program.methods.fundRewardPool(new anchor.BN(2000_000_000))
      .accountsPartial({
        signer: provider.wallet.publicKey,
        pool: pool_pda,
        rewardTokenMint: reward_mint,
        tokenProgram: TOKEN_PROGRAM_ID,
        rewardTokenVault: reward_vault_pda,
        adminRewardTokenAccount: admin_reward_token_account
      }).instruction()

    const latestblockhash = await connection.getLatestBlockhash();
    const tx = new anchor.web3.Transaction({
      feePayer: provider.wallet.publicKey,
      blockhash: latestblockhash.blockhash,
      lastValidBlockHeight: latestblockhash.lastValidBlockHeight
    }).add(fundIx)

    await anchor.web3.sendAndConfirmTransaction(
      connection,
      tx,
      [provider.wallet.payer],
      {skipPreflight: true}
    );

    const pool_account = await program.account.pool.fetch(pool_pda)
    const reward_vault_balance = await connection.getTokenAccountBalance(pool_account.rewardTokenVault)
    assert.equal(reward_vault_balance.value.uiAmount, 2000)
  })

  it("alice can stake", async () => {
    const alice_lst_token_account = get_lst_token_account(alice.publicKey);
    const stakeIx = await program.methods.stake(new anchor.BN(200_000_000_000))
      .accountsPartial({
        pool: pool_pda,
        lstTokenMint: lst_mint,
        signer: alice.publicKey,
        stakedTokenMint: staking_mint,
        tokenProgram: TOKEN_PROGRAM_ID,
        stakedTokenVault: staked_vault_pda,
        userStakedTokenAccount: alice_staking_token_account,
        userLstTokenAccount: alice_lst_token_account,
        systemProgram: anchor.web3.SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      }).instruction();

    const latestblockhash = await connection.getLatestBlockhash();
    const tx = new anchor.web3.Transaction({
      feePayer: alice.publicKey,
      blockhash: latestblockhash.blockhash,
      lastValidBlockHeight: latestblockhash.lastValidBlockHeight
    }).add(stakeIx)

    await anchor.web3.sendAndConfirmTransaction(
      connection,
      tx,
      [alice],
      {skipPreflight: true}
    );

    const lst_token_balance = await connection.getTokenAccountBalance(alice_lst_token_account)
    const alice_stake_account = await program.account.userStakeInfo.fetch(get_user_stake_info_pda(alice.publicKey))
    assert.equal(alice_stake_account.stakedAmount.toNumber(), 200_000_000_000)
    assert.equal(lst_token_balance.value.uiAmount, 200)
  })
});
