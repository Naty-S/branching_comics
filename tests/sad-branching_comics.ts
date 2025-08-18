import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

import { Keypair, PublicKey } from "@solana/web3.js";

import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { mplCore, MPL_CORE_PROGRAM_ID } from "@metaplex-foundation/mpl-core";

import { expect } from "chai";

import { BranchingComics } from "../target/types/branching_comics";
import * as helpers from "./helpers";
import { BN } from "bn.js";


describe("Fails branching_comics flow", () => {
  
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;
  const program = anchor.workspace.branchingComics as Program<BranchingComics>;
  const umi = createUmi(connection).use(mplCore());

  // const umi = createUmi(connection).use(mplCore());

  // Wallets and accounts to be used

  // Users
  let creator_wallet: Keypair;
  let reader_wallet: Keypair;
  let creator_user_pda: PublicKey;
  let reader_user_pda: PublicKey;

  // Configure comic
  const comic_title = "Test bad Comic";
  const comic_uri = "https://comic.io/bad-comic.json";
  const comic_0_collection = Keypair.generate();
  const comic_0_collection_authority_pda = helpers.get_comic_collection_authority(comic_0_collection.publicKey, program.programId);
  let comic_0_pda: PublicKey;

  // Configure chapters
  // let chapter_0_start_asset = generateSigner(umi);
  // Initial branch
  let chapter_0_start_mint = Keypair.generate();
  let chapter_0_start_pda: PublicKey;
  let chapter_0_1_mint = Keypair.generate();
  let chapter_0_1_pda: PublicKey;
  let chapter_0_end_mint = Keypair.generate();
  let chapter_0_end_pda: PublicKey;
  let choice_1_chapter_0_end_pda: PublicKey;
  let choice_2_chapter_0_end_pda: PublicKey;

  // Branch for choice 1
  let chapter_1_start_mint = Keypair.generate();
  let chapter_1_start_pda: PublicKey;

  // Branch for choice 2
  let chapter_2_start_mint = Keypair.generate();
  let chapter_2_start_pda: PublicKey;

  before(async () => {

    // Create and fund users
    creator_wallet = await helpers.gen_wallet(connection);
    reader_wallet = await helpers.gen_wallet(connection);
    creator_user_pda = helpers.get_user_pda(creator_wallet.publicKey, true, program.programId);
    reader_user_pda = helpers.get_user_pda(reader_wallet.publicKey, false, program.programId);

    // Configure comic
    comic_0_pda = helpers.get_comic_pda(comic_title, creator_wallet.publicKey, comic_0_collection.publicKey, program.programId);

    // Configure chapters
    chapter_0_start_pda = helpers.get_chapter_pda(comic_0_pda, chapter_0_start_mint.publicKey, program.programId);
    chapter_0_1_pda = helpers.get_chapter_pda(comic_0_pda, chapter_0_1_mint.publicKey, program.programId);
    chapter_0_end_pda = helpers.get_chapter_pda(comic_0_pda, chapter_0_end_mint.publicKey, program.programId);
    chapter_1_start_pda = helpers.get_chapter_pda(comic_0_pda, chapter_1_start_mint.publicKey, program.programId);
    chapter_2_start_pda = helpers.get_chapter_pda(comic_0_pda, chapter_2_start_mint.publicKey, program.programId);

    // Configure choices
    choice_1_chapter_0_end_pda = helpers.get_choice_pda(chapter_0_end_pda, "Bad Choice 1", program.programId);
    choice_2_chapter_0_end_pda = helpers.get_choice_pda(chapter_0_end_pda, "Bad Choice 2", program.programId);
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
      .then(signature => helpers.log(connection, signature, "User initialized"))
      .catch((err) => console.log(err));

    const creator_user_account = await program.account.user.fetch(creator_user_pda);

    expect(creator_user_account.creator, "Fail to init a creator").true;

    console.log("\t-> User initialized as creator successfully");

    await program.methods.initUser(false)
      .accountsPartial({
        user: reader_wallet.publicKey,
        userAccount: reader_user_pda,
        systemProgram: SYSTEM_PROGRAM_ID,
      })
      .signers([reader_wallet])
      .rpc()
      .then(signature => helpers.confirm(connection, signature))
      .then(signature => helpers.log(connection, signature, "User initialized"))
      .catch((err) => console.log(err));

    const reader_user_account = await program.account.user.fetch(reader_user_pda);

    expect(reader_user_account.creator, "Fail to init an user").false;

    console.log("\t-> Reader initialized successfully");
  });
  
  // ==========
  // Comic
  // ==========

  it.skip("No creator user fails to publish a comic", async () => {
    try {
      await program.methods.publishNewComic(comic_title, comic_uri)
        .accountsPartial({
          user: reader_wallet.publicKey,
          userAccount: reader_user_pda,
          collectionComic: comic_0_collection.publicKey,
          collectionComicAuthority: comic_0_collection_authority_pda,
          systemProgram: SYSTEM_PROGRAM_ID,
        })
        .signers([reader_wallet])
        .rpc()
        .then(signature => helpers.confirm(connection, signature))

      expect.fail("Transaction should have failed");

    } catch (err) {
      // error = err as anchor.AnchorError;
      // expect(error.error.errorCode.code).equal("UnauthorizedAdmin");
      console.log(err)
      expect(err.toString()).to.include("User is not a creator");
    }
  });
  it.skip("No creator user fails to unpublish a comic", async () => {
  });
  it("No creator user fails to republish a comic", async () => {
  });
  
  // ==========
  // Chapter
  // ==========

  it.skip("Creator user fails to creates a chapter of unpublished comic", async () => {
    
  });
  it.skip("No creator user fails to creates a chapter of published comic", async () => {
    
  });
  it.skip("User fails to list an unowned chapter for sale", async () => {
    
  });

  // ==========
  // Choice
  // ==========

  it.skip("Creator user fails to create choices of NO end of story branch/path", async () => {

  });
  it.skip("No creator user fails to create choices of end of story branch/path", async () => {

  });
  it.skip("Reader fails to select already selected choice for starting story", async () => {

  });
});
