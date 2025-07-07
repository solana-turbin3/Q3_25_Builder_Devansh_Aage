import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import { IDL, Q3PreReqTs } from "./programs/Turbin3_prereq";
import wallet from "./Turbin3-wallet.json";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

const MPL_CORE_PROGRAM_ID = new PublicKey(
  "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
);

const secretKey = bs58.decode(wallet.privateKey); // <- this will be 64 bytes
const keypair = Keypair.fromSecretKey(secretKey);

const connection = new Connection("https://api.devnet.solana.com");

const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment: "confirmed",
});

const program = new Program(IDL, provider);

const account_seeds = [Buffer.from("prereqs"), keypair.publicKey.toBuffer()];

const [account_key, account_bump] = PublicKey.findProgramAddressSync(
  account_seeds,
  program.programId
);

const mintCollection = new PublicKey(
  "5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2"
);

const authority_seeds = [Buffer.from("collection"), mintCollection.toBuffer()];

const [authority_key, authority_bump] = PublicKey.findProgramAddressSync(
  authority_seeds,
  program.programId
);

const mintTs = Keypair.generate();

// (async () => {
//   try {
//     const txhash = await program.methods
//       .initialize("Devansh-Aage")
//       .accountsPartial({
//         user: keypair.publicKey,
//         account: account_key,
//         system_program: SYSTEM_PROGRAM_ID,
//       })
//       .signers([keypair])
//       .rpc();
//     console.log(
//       `Success! Check out TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`
//     );
//   } catch (error) {
//     console.error(`Oops, something went wrong: ${error}`);
//   }
// })();

(async () => {
  try {
    const txhash = await program.methods
      .submitTs()
      .accountsPartial({
        user: keypair.publicKey,
        account: account_key,
        mint: mintTs.publicKey,
        collection: mintCollection,
        authority: authority_key,
        mpl_core_program: MPL_CORE_PROGRAM_ID,
        system_program: SYSTEM_PROGRAM_ID,
      })
      .signers([keypair, mintTs])
      .rpc();

    console.log(
      `Success! Check out TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`
    );
  } catch (error) {
    console.error(`Oops, something went wrong: ${error}`);
  }
})();
