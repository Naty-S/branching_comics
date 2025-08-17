import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
// import {
//     KeypairSigner
//   , PublicKey
//   , Umi
// } from "@metaplex-foundation/umi";
import {
    LAMPORTS_PER_SOL
  , Keypair
  , PublicKey
  , Connection
  , sendAndConfirmTransaction
  , Transaction
} from "@solana/web3.js";



export const confirm = async (connection: Connection, signature: string): Promise<string> => {
  
  const block = await connection.getLatestBlockhash();
  
  await connection.confirmTransaction({
    signature,
    ...block,
  });
  
  return signature;
};

export const log = (connection: Connection, signature: string, info: string): string => {
  
  console.log(
    `\n${info}:\
    \nTransaction: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
  );
  
  return signature;
};

export const log_anchor = async (connection: Connection, signature: string,): Promise<string> => {
  
  const info = await connection.getParsedTransaction(signature, { commitment: "confirmed" });

  console.log("Anchor Logs:", info?.meta?.logMessages);

  return signature;
};

export const gen_wallet = async (connection: Connection): Promise<Keypair> => {
  
  const keypair = Keypair.generate();

  await connection.requestAirdrop(
    keypair.publicKey,
    5 * LAMPORTS_PER_SOL
  )
  .then(signature => confirm(connection, signature))
  .then(signature => log(connection, signature, "Wallet created and funded"));

  console.log(`Wallet Public Key: ${keypair.publicKey.toBase58()}`);

  return keypair;
};

// export const close_wallet = async (
//   connection: Connection,
//   wallet: Keypair
// ): Promise<anchor.web3.TransactionSignature> => {

//   // Transferir casi todo el saldo (dejando ~0.002 SOL para fees)
//   const transferTx = new Transaction().add(
//     SystemProgram.transfer({
//       fromPubkey: accountToClose,
//       toPubkey: receiver,
//       lamports: (await connection.getBalance(accountToClose)) - 200000 // Deja 0.002 SOL
//     })
//   );

//   await connection.sendTransaction(transferTx, [owner]);

//   const closeTx = new Transaction().add(
//     SystemProgram.closeAccount({
//       accountPubkey: accountToClose,
//       destinationPubkey: receiver,
//       authorityPubkey: owner.publicKey
//     })
//   );

//   await connection.sendTransaction(closeTx, [owner]);
// };

export const get_user_pda = (
  creator: PublicKey,
  is_creator: Boolean,
  program_id: PublicKey
): PublicKey => {

  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("user"),
      creator.toBuffer(),
      Buffer.from(is_creator.toString())
    ],
    program_id
  )[0];
};

export const get_comic_pda = (
  title: string,
  creator: PublicKey,
  collection: PublicKey,
  program_id: PublicKey
): PublicKey => {
  
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("comic"),
      collection.toBuffer(),
      creator.toBuffer(),
      Buffer.from(title)
    ],
    program_id
  )[0];
};

export const get_comic_collection_authority = (
  collection: PublicKey,
  program_id: PublicKey
): PublicKey => {
  
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("authority"),
      collection.toBuffer(),
    ],
    program_id
  )[0];
};

export const get_chapter_pda = (
  comic: PublicKey,
  mint: PublicKey,
  program_id: PublicKey
): PublicKey => {
  
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("chapter"),
      mint.toBuffer(),
      comic.toBuffer()
    ],
    program_id
  )[0];
};

export const get_choice_pda = (
  chapter: PublicKey,
  choice: String,
  program_id: PublicKey
): PublicKey => {

  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("choice"),
      chapter.toBuffer(),
      Buffer.from(choice)
    ],
    program_id
  )[0];
};
