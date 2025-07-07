import { Keypair, Connection, Commitment } from "@solana/web3.js";
import {createMint} from "@solana/spl-token"
import path from "node:path";
import * as fs from "node:fs"

const walletPath = path.join(__dirname,"./wallet", "/turbin3-wallet.json");
const wallet = JSON.parse(fs.readFileSync(walletPath, "utf8"));
// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);



(async () => {
    try {
      const mint=await createMint(connection,keypair,keypair.publicKey,keypair.publicKey,6)
        console.log("Mint Address: ",mint)
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
