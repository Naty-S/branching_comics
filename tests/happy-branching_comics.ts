import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

import {
    ASSOCIATED_TOKEN_PROGRAM_ID
  , TOKEN_PROGRAM_ID
  , getAssociatedTokenAddressSync
} from "@solana/spl-token";
import {
  LAMPORTS_PER_SOL
  , Keypair
  , PublicKey
  , Connection
  , sendAndConfirmTransaction
  , Transaction
} from "@solana/web3.js";

import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
    KeypairSigner
  // , PublicKey
  , base58
  , createSignerFromKeypair
  , generateSigner
  , keypairIdentity
  , signerIdentity
  , sol
  , percentAmount
} from "@metaplex-foundation/umi";
import {
  createPluginV2,
  createV1,
  transferV1,
  fetchAssetV1,
  mplCore,
  pluginAuthority,
  MPL_CORE_PROGRAM_ID
} from "@metaplex-foundation/mpl-core";

import { expect } from "chai";

import { BranchingComics } from "../target/types/branching_comics";
import * as helpers from "./helpers";


describe("Common branching_comics flow", () => {
  
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;
  const program = anchor.workspace.branchingComics as Program<BranchingComics>;
  const umi = createUmi(connection).use(mplCore());

  // Wallets and accounts to be used
  
  // Users
  let creator_wallet: Keypair;
  let reader_wallet: Keypair;
  let creator_user_pda: PublicKey;
  let reader_user_pda: PublicKey;

  // Configure comic
  const comic_title = "Test Comic";
  const comic_uri = "https://comic.io/comic.json";
  const comic_0_collection = Keypair.generate();
  const comic_0_collection_authority_pda = helpers.get_comic_collection_authority(comic_0_collection.publicKey, program.programId);
  let comic_0_pda: PublicKey;

  // Configure chapters
  let chapter_0_start_mint: Keypair; // generateSigner(umi)
  let chapter_0_start_pda: PublicKey;
  let chapter_0_1_mint: Keypair;
  let chapter_0_1_pda: PublicKey;
  let chapter_0_end_mint: Keypair;
  let chapter_0_end_pda: PublicKey;
  let choice_0_chapter_0_end_pda: PublicKey;
  let choice_1_chapter_0_end_pda: PublicKey;
  let choice_2_chapter_0_end_pda: PublicKey;
  let chapter_1_start_mint: Keypair;
  let chapter_1_start_pda: PublicKey;
  let chapter_1_1_mint: Keypair;
  let chapter_1_1_pda: PublicKey;
  let chapter_1_end_mint: Keypair;
  let chapter_1_end_pda: PublicKey;
  let choice_0_chapter_1_end_pda: PublicKey;
  let choice_1_chapter_1_end_pda: PublicKey;
  let choice_2_chapter_1_end_pda: PublicKey;


  before(async () => {

    // Create and fund users
    creator_wallet = await helpers.gen_wallet(connection);
    reader_wallet = await helpers.gen_wallet(connection);
    creator_user_pda = helpers.get_user_pda(creator_wallet.publicKey, true, program.programId);
    reader_user_pda = helpers.get_user_pda(reader_wallet.publicKey, false, program.programId);

    // Configure comic
    comic_0_pda = helpers.get_comic_pda(comic_title, creator_wallet.publicKey, comic_0_collection.publicKey, program.programId);
  });

  after(async () => {
    
    // Close accounts and wallets
  });

  // ==========
  // Users
  // ==========

  it("Init creator and reader (buyer)", async () => {
    
    await program.methods.initUser(true)
      .accountsPartial({
        user: creator_wallet.publicKey,
        userAccount: creator_user_pda,
        systemProgram: SYSTEM_PROGRAM_ID,
      })
      .signers([creator_wallet])
      .rpc()
      .then(signature => helpers.confirm(connection, signature))
      .then(signature => helpers.log(connection, signature, "Creator user initialized"));

    const creator_user_account = await program.account.user.fetch(creator_user_pda);

    expect(creator_user_account.creator, "Fail to init a creator").to.be.true;

    console.log("User initialized as creator successfully");

    await program.methods.initUser(false)
      .accountsPartial({
        user: reader_wallet.publicKey,
        userAccount: reader_user_pda,
        systemProgram: SYSTEM_PROGRAM_ID,
      })
      .signers([reader_wallet])
      .rpc()
      .then(signature => helpers.confirm(connection, signature))
      .then(signature => helpers.log(connection, signature, "User initialized"));

    const reader_user_account = await program.account.user.fetch(reader_user_pda);

    expect(reader_user_account.creator, "Fail to init an user").to.be.false;

    console.log("User initialized successfully");
  });
  
  // ==========
  // Comic
  // ==========

  it("Creator user publish a comic", async () => {

    await program.methods.publishNewComic(comic_title, comic_uri)
      .accountsPartial({
        user: creator_wallet.publicKey,
        userAccount: creator_user_pda,
        collectionComic: comic_0_collection.publicKey,
        collectionComicAuthority: comic_0_collection_authority_pda,
        systemProgram: SYSTEM_PROGRAM_ID
      })
      .signers([creator_wallet, comic_0_collection])
      .rpc()
      .then(signature => helpers.confirm(connection, signature))
      .then(signature => helpers.log(connection, signature, "Comic published"));

      const comic_account = await program.account.comic.fetch(comic_0_pda);

    console.log(comic_account.creator.toBase58(), "Comic creator");
    console.log(creator_wallet.publicKey.toBase58(), "Comic creator wallet");
    expect(comic_account.creator.equals(creator_wallet.publicKey), "Comic creator is not correct");
    expect(comic_account.title).to.equal(comic_title, "Comic title is not correct");
    expect(comic_account.collection.equals(comic_0_collection.publicKey), "Comic collection is not correct");
    expect(comic_account.published, "Fail to publish comic").to.be.true;

    console.log("Comic published successfully");
  });
  it("Creator user unpublish a comic", async () => {
  });
  it("Creator user republish a comic", async () => {
  });
  
  // ==========
  // Chapter
  // ==========

  it("Creator user creates a chapter of published comic", async () => {
    
  });
  it("User list a chapter for sale", async () => {
    
  });
  it("Reader purchase chapter", async () => {
    
  });

  // ==========
  // Choice
  // ==========

  it("Creator user creates choices of end of story branch/path", async () => {

  });
  it("Reader selects a choice", async () => {
    
  });
});
