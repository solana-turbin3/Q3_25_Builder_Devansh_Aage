import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

import * as fs from "node:fs"
import path from "node:path";

const walletPath = path.join(__dirname,"./wallet", "/turbin3-wallet.json");
const wallet = JSON.parse(fs.readFileSync(walletPath, "utf8"));

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000n;
// Mint address
const mint = new PublicKey("Cnn74hnQtfATFU41AqjkP8Bpwv58sVE35YsR4Bc7HL4S");

// Recipient address
const to = new PublicKey("2G3ogWyzrdiUeZhFja3izVfDwtwvLMF3ZeCHkNrVPHLW");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromWallet=await getOrCreateAssociatedTokenAccount(connection,keypair,mint,keypair.publicKey)
        // Get the token account of the toWallet address, and if it does not exist, create it
        const toWallet= await getOrCreateAssociatedTokenAccount(connection,keypair,mint,to)
        // Transfer the new token to the "toTokenAccount" we just created
        const tx=await transfer(
            connection,
            keypair,
            fromWallet.address,
            toWallet.address,
            keypair.publicKey,
            10n*token_decimals
        )
        console.log(`tx is ${tx}`)
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();