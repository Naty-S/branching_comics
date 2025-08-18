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


describe("Common branching_comics flow", () => {
  
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;
  const program = anchor.workspace.branchingComics as Program<BranchingComics>;
  // const umi = createUmi(connection).use(mplCore());

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
  // let chapter_1_1_mint = Keypair.generate();
  // let chapter_1_1_pda: PublicKey;
  // let chapter_1_end_mint = Keypair.generate();
  // let chapter_1_end_pda: PublicKey;

  // Branch for choice 2
  let chapter_2_start_mint = Keypair.generate();
  let chapter_2_start_pda: PublicKey;
  // let chapter_2_1_mint = Keypair.generate();
  // let chapter_2_1_pda: PublicKey;
  // let chapter_2_end_mint = Keypair.generate();
  // let chapter_2_end_pda: PublicKey;
  // let choice_0_chapter_1_end_pda: PublicKey;
  // let choice_1_chapter_1_end_pda: PublicKey;
  // let choice_2_chapter_1_end_pda: PublicKey;


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
    // chapter_1_1_pda = helpers.get_chapter_pda(comic_0_pda, chapter_1_1_mint.publicKey, program.programId);
    // chapter_1_end_pda = helpers.get_chapter_pda(comic_0_pda, chapter_1_end_mint.publicKey, program.programId);
    chapter_2_start_pda = helpers.get_chapter_pda(comic_0_pda, chapter_2_start_mint.publicKey, program.programId);
    // chapter_2_1_pda = helpers.get_chapter_pda(comic_0_pda, chapter_2_1_mint.publicKey, program.programId);
    // chapter_2_end_pda = helpers.get_chapter_pda(comic_0_pda, chapter_2_end_mint.publicKey, program.programId);

    // Configure choices
    choice_1_chapter_0_end_pda = helpers.get_choice_pda(chapter_0_end_pda, "Choice 1", program.programId);
    choice_2_chapter_0_end_pda = helpers.get_choice_pda(chapter_0_end_pda, "Choice 2", program.programId);
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

  it("Creator user publish comic", async () => {

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
      .then(signature => helpers.log(connection, signature, "Comic published"))
      .catch((err) => console.log(err));

    // const collection = await fetchCollectionWithRetry(eventKeypair.publicKey);
    const comic_account = await program.account.comic.fetch(comic_0_pda);

    expect(comic_account.creator.toString()).equal(creator_wallet.publicKey.toString(), "Comic creator is not correct");
    expect(comic_account.title).equal(comic_title, "Comic title is not correct");
    expect(comic_account.collection.toString()).equal(comic_0_collection.publicKey.toString(), "Comic collection is not correct");
    expect(comic_account.published, "Fail to publish comic").true;
    // expect(collection.name).to.equal(comic.name);

    console.log("\t-> Comic published tests successfull");
  });
  it.skip("Creator user unpublish comic", async () => {
  });
  it.skip("Creator user republish comic", async () => {

    await program.methods.republishComic()
      .accountsPartial({
        user: creator_wallet.publicKey,
        userAccount: creator_user_pda,
        comic: comic_0_pda,
        collectionComic: comic_0_collection.publicKey,
        collectionComicAuthority: comic_0_collection_authority_pda,
        systemProgram: SYSTEM_PROGRAM_ID
      })
      .signers([creator_wallet, comic_0_collection])
      .rpc()
      .then(signature => helpers.confirm(connection, signature))
      .then(signature => helpers.log(connection, signature, "Comic republished"))
      .catch((err) => console.log(err));

    // const collection = await fetchCollectionWithRetry(eventKeypair.publicKey);
    const comic_account = await program.account.comic.fetch(comic_0_pda);

    expect(comic_account.published, "Fail to republish comic").true;

    console.log("Comic republished test successfull");
  });
  
  // ==========
  // Chapters
  // ==========

  it("Creator user sets up starting story", async () => {

    await program.methods.createChapter(true, "Chapter 0.start", "https://comic.io/chapter_0_start.json")
      .accountsPartial({
        user: creator_wallet.publicKey,
        userAccount: creator_user_pda,
        comic: comic_0_pda,
        parent: null,
        chapter: chapter_0_start_pda,
        mint: chapter_0_start_mint.publicKey,
        collectionComic: comic_0_collection.publicKey,
        systemProgram: SYSTEM_PROGRAM_ID,
        mplCoreProgram: MPL_CORE_PROGRAM_ID,
      })
      .signers([creator_wallet, chapter_0_start_mint])
      .rpc()
      .then(signature => helpers.confirm(connection, signature))
      // .then(signature => helpers.log_anchor(connection, signature))
      .then(signature => helpers.log(connection, signature, "Chapter created"))
      .catch((err) => console.log(err));

    // const fetchedAsset = await fetchAssetV1(umi, publicKey(chapter_0_start_mint.publicKey))
    // console.log("\nAsset fetched:\n", fetchedAsset)

    let chapter_account = await program.account.chapter.fetch(chapter_0_start_pda);

    expect(chapter_account.owner.toString()).equal(creator_wallet.publicKey.toString(), "Owner is not creator");
    expect(chapter_account.comic.toString()).equal(comic_0_pda.toString(), "Chapter not belong to this comic");
    expect(chapter_account.mint.toString()).equal(chapter_0_start_mint.publicKey.toString(), "Mint is not from this chapter");
    expect(chapter_account.next).null;
    expect(chapter_account.start).true;
    expect(chapter_account.choices).empty;
    expect(chapter_account.price.toNumber()).equal(0);
    // expect(chapter_start_account.attributes)

    console.log("\t-> Start chapter tested successfully");

    await program.methods.createChapter(false, "Chapter 0.1", "https://comic.io/chapter_0_1.json")
      .accountsPartial({
        user: creator_wallet.publicKey,
        userAccount: creator_user_pda,
        comic: comic_0_pda,
        parent: chapter_0_start_pda,
        chapter: chapter_0_1_pda,
        mint: chapter_0_1_mint.publicKey,
        collectionComic: comic_0_collection.publicKey,
        systemProgram: SYSTEM_PROGRAM_ID,
        mplCoreProgram: MPL_CORE_PROGRAM_ID,
      })
      .signers([creator_wallet, chapter_0_1_mint])
      .rpc()
      .then(signature => helpers.confirm(connection, signature))
      .then(signature => helpers.log(connection, signature, "Chapter created"))
      .catch((err) => console.log(err));

    chapter_account = await program.account.chapter.fetch(chapter_0_1_pda);
    let parent_account = await program.account.chapter.fetch(chapter_0_start_pda);

    expect(chapter_account.owner.toString()).equal(creator_wallet.publicKey.toString(), "Owner is not creator");
    expect(chapter_account.comic.toString()).equal(comic_0_pda.toString(), "Chapter not belong to this comic");
    expect(chapter_account.mint.toString()).equal(chapter_0_1_mint.publicKey.toString(), "Mint is not from this chapter");
    expect(chapter_account.next).null;
    expect(chapter_account.start).false;
    expect(chapter_account.choices).empty;
    expect(chapter_account.price.toNumber()).equal(0);
    expect(parent_account.next.toString()).equal(chapter_0_1_pda.toString(), "Chapter 1 not linked to parent");
    // expect(chapter_account.attributes)
    console.log("\t-> Chapter 1 tested successfully");

    await program.methods.createChapter(false, "Chapter 0.end", "https://comic.io/chapter_0_end.json")
      .accountsPartial({
        user: creator_wallet.publicKey,
        userAccount: creator_user_pda,
        comic: comic_0_pda,
        parent: chapter_0_1_pda,
        chapter: chapter_0_end_pda,
        mint: chapter_0_end_mint.publicKey,
        collectionComic: comic_0_collection.publicKey,
        systemProgram: SYSTEM_PROGRAM_ID,
        mplCoreProgram: MPL_CORE_PROGRAM_ID,
      })
      .signers([creator_wallet, chapter_0_end_mint])
      .rpc()
      .then(signature => helpers.confirm(connection, signature))
      .then(signature => helpers.log(connection, signature, "Chapter created"))
      // .then(signature => helpers.log_anchor(connection, signature))
      .catch((err) => console.log(err));

    chapter_account = await program.account.chapter.fetch(chapter_0_end_pda);
    parent_account = await program.account.chapter.fetch(chapter_0_1_pda);

    expect(chapter_account.owner.toString()).equal(creator_wallet.publicKey.toString(), "Owner is not creator");
    expect(chapter_account.comic.toString()).equal(comic_0_pda.toString(), "Chapter not belong to this comic");
    expect(chapter_account.mint.toString()).equal(chapter_0_end_mint.publicKey.toString(), "Mint is not from this chapter");
    expect(chapter_account.next).null;
    expect(chapter_account.start).false;
    expect(chapter_account.choices).empty;
    expect(chapter_account.price.toNumber()).equal(0);
    expect(parent_account.next.toString()).equal(chapter_0_end_pda.toString(), "End chapter not linked to parent");
    // expect(chapter_account.attributes)

    console.log("\t-> End Chapter tested successfully");
  });
  it("Creator user sets up first branch", async () => {
    
    await program.methods.createChapter(true, "Chapter 1.start", "https://comic.io/chapter_1_start.json")
      .accountsPartial({
        user: creator_wallet.publicKey,
        userAccount: creator_user_pda,
        comic: comic_0_pda,
        parent: null,
        chapter: chapter_1_start_pda,
        mint: chapter_1_start_mint.publicKey,
        collectionComic: comic_0_collection.publicKey,
        systemProgram: SYSTEM_PROGRAM_ID,
        mplCoreProgram: MPL_CORE_PROGRAM_ID,
      })
      .signers([creator_wallet, chapter_1_start_mint])
      .rpc()
      .then(signature => helpers.confirm(connection, signature))
      .then(signature => helpers.log(connection, signature, "Chapter created"))
      .catch((err) => console.log(err));

    let chapter_account = await program.account.chapter.fetch(chapter_1_start_pda);

    expect(chapter_account.owner.toString()).equal(creator_wallet.publicKey.toString(), "Owner is not creator");
    expect(chapter_account.comic.toString()).equal(comic_0_pda.toString(), "Chapter not belong to this comic");
    expect(chapter_account.mint.toString()).equal(chapter_1_start_mint.publicKey.toString(), "Mint is not from this chapter");
    expect(chapter_account.next).null;
    expect(chapter_account.start).true;
    expect(chapter_account.choices).empty;
    expect(chapter_account.price.toNumber()).equal(0);
    // expect(chapter_start_account.attributes)

    console.log("\t-> Start chapter tested successfully");

    // await program.methods.createChapter(false, "Chapter 1.1", "https://comic.io/chapter_1_1.json")
    //   .accountsPartial({
    //     user: creator_wallet.publicKey,
    //     userAccount: creator_user_pda,
    //     comic: comic_0_pda,
    //     parent: chapter_1_start_pda,
    //     chapter: chapter_1_1_pda,
    //     mint: chapter_1_1_mint.publicKey,
    //     collectionComic: comic_0_collection.publicKey,
    //     systemProgram: SYSTEM_PROGRAM_ID,
    //     mplCoreProgram: MPL_CORE_PROGRAM_ID,
    //   })
    //   .signers([creator_wallet, chapter_1_1_mint])
    //   .rpc()
    //   .then(signature => helpers.confirm(connection, signature))
    //   .then(signature => helpers.log(connection, signature, "Chapter created"));

    // chapter_account = await program.account.chapter.fetch(chapter_1_1_pda);
    // let parent_account = await program.account.chapter.fetch(chapter_1_start_pda);

    // expect(chapter_account.owner.equals(creator_wallet.publicKey), "Owner is not creator");
    // expect(chapter_account.comic.equals(comic_0_pda), "Chapter not belong to this comic");
    // expect(chapter_account.mint.equals(chapter_1_1_pda), "Mint is not from this chapter");
    // expect(chapter_account.next).null;
    // expect(chapter_account.start).false;
    // expect(chapter_account.choices).empty;
    // expect(chapter_account.price.toNumber()).equal(0);
    // expect(parent_account.next.toString()).equal(chapter_1_1_pda.toString(), "Chapter 1 not linked to parent");
    // // expect(chapter_account.attributes)

    // console.log("\t-> Chapter 1 tested successfully");

    // await program.methods.createChapter(false, "Chapter 0.end", "https://comic.io/chapter_0_end.json")
    //   .accountsPartial({
    //     user: creator_wallet.publicKey,
    //     userAccount: creator_user_pda,
    //     comic: comic_0_pda,
    //     parent: chapter_1_1_pda,
    //     chapter: chapter_1_end_pda,
    //     mint: chapter_1_end_mint.publicKey,
    //     collectionComic: comic_0_collection.publicKey,
    //     systemProgram: SYSTEM_PROGRAM_ID,
    //     mplCoreProgram: MPL_CORE_PROGRAM_ID,
    //   })
    //   .signers([creator_wallet, chapter_1_end_mint])
    //   .rpc()
    //   .then(signature => helpers.confirm(connection, signature))
    //   .then(signature => helpers.log(connection, signature, "Chapter created"));

    // chapter_account = await program.account.chapter.fetch(chapter_1_end_pda);
    // parent_account = await program.account.chapter.fetch(chapter_1_1_pda);

    // expect(chapter_account.owner.equals(creator_wallet.publicKey), "Owner is not creator");
    // expect(chapter_account.comic.equals(comic_0_pda), "Chapter not belong to this comic");
    // expect(chapter_account.mint.equals(chapter_1_end_pda), "Mint is not from this chapter");
    // expect(chapter_account.next).null;
    // expect(chapter_account.start).false;
    // expect(chapter_account.choices).empty;
    // expect(chapter_account.price.toNumber()).equal(0);
    // expect(parent_account.next.toString()).equal(chapter_1_end_pda.toString(), "End chapter not linked to parent");
    // // expect(chapter_account.attributes)

    // console.log("\t-> End Chapter tested successfully");
  });
  it("Creator user sets up sencond branch", async () => {
    
    await program.methods.createChapter(true, "Chapter 2.start", "https://comic.io/chapter_2_start.json")
      .accountsPartial({
        user: creator_wallet.publicKey,
        userAccount: creator_user_pda,
        comic: comic_0_pda,
        parent: null,
        chapter: chapter_2_start_pda,
        mint: chapter_2_start_mint.publicKey,
        collectionComic: comic_0_collection.publicKey,
        systemProgram: SYSTEM_PROGRAM_ID,
        mplCoreProgram: MPL_CORE_PROGRAM_ID,
      })
      .signers([creator_wallet, chapter_2_start_mint])
      .rpc()
      .then(signature => helpers.confirm(connection, signature))
      .then(signature => helpers.log(connection, signature, "Chapter created"))
      .catch((err) => console.log(err));

    let chapter_account = await program.account.chapter.fetch(chapter_2_start_pda);

    expect(chapter_account.owner.toString()).equal(creator_wallet.publicKey.toString(), "Owner is not creator");
    expect(chapter_account.comic.toString()).equal(comic_0_pda.toString(), "Chapter not belong to this comic");
    expect(chapter_account.mint.toString()).equal(chapter_2_start_mint.publicKey.toString(), "Mint is not from this chapter");
    expect(chapter_account.next).null;
    expect(chapter_account.start).true;
    expect(chapter_account.choices).empty;
    expect(chapter_account.price.toNumber()).equal(0);
    // expect(chapter_start_account.attributes)

    console.log("\t-> Start chapter tested successfully");

    // await program.methods.createChapter(false, "Chapter 2.1", "https://comic.io/chapter_2_1.json")
    //   .accountsPartial({
    //     user: creator_wallet.publicKey,
    //     userAccount: creator_user_pda,
    //     comic: comic_0_pda,
    //     parent: chapter_2_start_pda,
    //     chapter: chapter_2_1_pda,
    //     mint: chapter_2_1_mint.publicKey,
    //     collectionComic: comic_0_collection.publicKey,
    //     systemProgram: SYSTEM_PROGRAM_ID,
    //     mplCoreProgram: MPL_CORE_PROGRAM_ID,
    //   })
    //   .signers([creator_wallet, chapter_2_1_mint])
    //   .rpc()
    //   .then(signature => helpers.confirm(connection, signature))
    //   .then(signature => helpers.log(connection, signature, "Chapter created"));

    // chapter_account = await program.account.chapter.fetch(chapter_2_1_pda);
    // let parent_account = await program.account.chapter.fetch(chapter_2_start_pda);

    // expect(chapter_account.owner.equals(creator_wallet.publicKey), "Owner is not creator");
    // expect(chapter_account.comic.equals(comic_0_pda), "Chapter not belong to this comic");
    // expect(chapter_account.mint.equals(chapter_2_1_pda), "Mint is not from this chapter");
    // expect(chapter_account.next).null;
    // expect(chapter_account.start).false;
    // expect(chapter_account.choices).empty;
    // expect(chapter_account.price.toNumber()).equal(0);
    // expect(parent_account.next.toString()).equal(chapter_2_1_pda.toString(), "Chapter 1 not linked to parent");
    // // expect(chapter_account.attributes)

    // console.log("\t-> Chapter 1 tested successfully");

    // await program.methods.createChapter(false, "Chapter 2.end", "https://comic.io/chapter_2_end.json")
    //   .accountsPartial({
    //     user: creator_wallet.publicKey,
    //     userAccount: creator_user_pda,
    //     comic: comic_0_pda,
    //     parent: chapter_2_1_pda,
    //     chapter: chapter_2_end_pda,
    //     mint: chapter_2_end_mint.publicKey,
    //     collectionComic: comic_0_collection.publicKey,
    //     systemProgram: SYSTEM_PROGRAM_ID,
    //     mplCoreProgram: MPL_CORE_PROGRAM_ID,
    //   })
    //   .signers([creator_wallet, chapter_2_end_mint])
    //   .rpc()
    //   .then(signature => helpers.confirm(connection, signature))
    //   .then(signature => helpers.log(connection, signature, "Chapter created"));

    // chapter_account = await program.account.chapter.fetch(chapter_2_end_pda);
    // parent_account = await program.account.chapter.fetch(chapter_2_1_pda);

    // expect(chapter_account.owner.equals(creator_wallet.publicKey), "Owner is not creator");
    // expect(chapter_account.comic.equals(comic_0_pda), "Chapter not belong to this comic");
    // expect(chapter_account.mint.equals(chapter_2_end_pda), "Mint is not from this chapter");
    // expect(chapter_account.next).null;
    // expect(chapter_account.start).false;
    // expect(chapter_account.choices).empty;
    // expect(chapter_account.price.toNumber()).equal(0);
    // expect(parent_account.next.toString()).equal(chapter_2_end_pda.toString(), "End chapter not linked to parent");
    // // expect(chapter_account.attributes)

    // console.log("\t-> End Chapter tested successfully");
  });
  it("User list chapter for sale", async () => {
    
    const chapter_price = new BN(1);
    
    await program.methods.listChapter(chapter_price)
      .accountsPartial({
        user: creator_wallet.publicKey,
        userAccount: creator_user_pda,
        comic: comic_0_pda,
        chapter: chapter_0_end_pda,
        mint: chapter_0_end_mint.publicKey,
        collectionComic: comic_0_collection.publicKey,
        systemProgram: SYSTEM_PROGRAM_ID,
        mplCoreProgram: MPL_CORE_PROGRAM_ID,
      })
      .signers([creator_wallet])
      .rpc()
      // .then(signature => helpers.log_anchor(connection, signature))
      .then(signature => helpers.confirm(connection, signature))
      .then(signature => helpers.log(connection, signature, "Chapter listed for sale"))
      .catch((err) => console.log(err));
    
    const chapter_account = await program.account.chapter.fetch(chapter_0_end_pda);

    expect(chapter_account.price.toNumber()).equal(chapter_price.toNumber(), "Chapter price is not set correctly");

    console.log("\t-> Chapter listed for sale successfully");
  });
  it("Reader purchase chapter", async () => {
    
    await program.methods.purchaseChapter()
      .accountsPartial({
        buyer: reader_wallet.publicKey,
        seller: creator_wallet.publicKey,
        chapter: chapter_0_end_pda,
        mint: chapter_0_end_mint.publicKey,
        collectionComic: comic_0_collection.publicKey,
        systemProgram: SYSTEM_PROGRAM_ID,
        mplCoreProgram: MPL_CORE_PROGRAM_ID,
      })
      .signers([reader_wallet])
      .rpc()
      .then(signature => helpers.confirm(connection, signature))
      .then(signature => helpers.log(connection, signature, "Chapter purchased"))
      .catch((err) => console.log(err));

    const chapter_account = await program.account.chapter.fetch(chapter_0_end_pda);

    // console.log(chapter_account.owner.toBytes())
    // console.log(reader_wallet.publicKey.toBytes())
    // expect(chapter_account.owner.equals(reader_wallet.publicKey), "Chapter's owners didn't change to reader");
    expect(chapter_account.owner.toString()).equal(reader_wallet.publicKey.toString(), "Chapter's owners didn't change to buyer");

    console.log("\t-> Chapter purchased successfully");
  });

  // ==========
  // Choices
  // ==========

  it("Creator user sets up choices for starting story", async () => {

    await program.methods.createChoice("Choice 1")
      .accountsPartial({
        user: creator_wallet.publicKey,
        userAccount: creator_user_pda,
        chapter: chapter_0_end_pda,
        nextChapter: chapter_1_start_pda,
        choice: choice_1_chapter_0_end_pda,
        systemProgram: SYSTEM_PROGRAM_ID,
      })
      .signers([creator_wallet])
      .rpc()
      .then(signature => helpers.confirm(connection, signature))
      .then(signature => helpers.log(connection, signature, "Choice created"))
      .catch((err) => console.log(err));

    let choice_account = await program.account.choice.fetch(choice_1_chapter_0_end_pda);

    expect(choice_account.choice).equal("Choice 1", "Choice text is not correct");
    expect(choice_account.chapter.toString()).equal(chapter_0_end_pda.toString(), "Choice chapter is not correct");
    expect(choice_account.nextChapter.toString()).equal(chapter_1_start_pda.toString(), "Choice next chapter is not correct");

    console.log("\t-> Choice 1 tested successfully");

    await program.methods.createChoice("Choice 2")
      .accountsPartial({
        user: creator_wallet.publicKey,
        userAccount: creator_user_pda,
        chapter: chapter_0_end_pda,
        nextChapter: chapter_2_start_pda,
        choice: choice_2_chapter_0_end_pda,
        systemProgram: SYSTEM_PROGRAM_ID,
      })
      .signers([creator_wallet])
      .rpc()
      .then(signature => helpers.confirm(connection, signature))
      .then(signature => helpers.log(connection, signature, "Choice created"))
      .catch((err) => console.log(err));

    choice_account = await program.account.choice.fetch(choice_2_chapter_0_end_pda);

    expect(choice_account.choice).equal("Choice 2", "Choice text is not correct");
    expect(choice_account.chapter.toString()).equal(chapter_0_end_pda.toString(), "Choice chapter is not correct");
    expect(choice_account.nextChapter.toString()).equal(chapter_2_start_pda.toString(), "Choice next chapter is not correct");

    console.log("\t-> Choice 2 tested successfully");
  });
  it("Reader selects choice that leads to branch 1", async () => {
    
    await program.methods.makeChoice("Choice 1")
      .accountsPartial({
        user: reader_wallet.publicKey,
        chapter: chapter_0_end_pda,
        choice: choice_1_chapter_0_end_pda,
        systemProgram: SYSTEM_PROGRAM_ID,
      })
      .signers([reader_wallet])
      .rpc()
      .then(signature => helpers.confirm(connection, signature))
      .then(signature => helpers.log(connection, signature, "Choice selected"))
      // .then(signature => helpers.log_anchor(connection, signature))
      .catch((err) => console.log(err));

    let chapter_account = await program.account.chapter.fetch(chapter_0_end_pda);

    expect(chapter_account.next.toString()).equal(chapter_1_start_pda.toString(), "Choice didn't change next chapter");

    console.log("\t-> Choice selection tested successfully");
  });
  it("Reader can't select choice that leads to branch 2 after selecting branch 1", async () => {

    let error: anchor.AnchorError | null = null;

    try {
      await program.methods.makeChoice("Choice 2")
        .accountsPartial({
          user: reader_wallet.publicKey,
          chapter: chapter_0_end_pda,
          choice: choice_1_chapter_0_end_pda,
          systemProgram: SYSTEM_PROGRAM_ID,
        })
        .signers([reader_wallet])
        .rpc()

      expect.fail("Transaction should have failed");

    } catch (err) {
      // error = err as anchor.AnchorError;
      // expect(error.error.errorCode.code).equal("UnauthorizedAdmin");
      expect(err.toString()).to.include("Choice already selected in this chapter");
    }
  });
  it.skip("Reader can list chapter after purchase", async () => {
    
  });
});
