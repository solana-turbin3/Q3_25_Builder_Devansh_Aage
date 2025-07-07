import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"
import path from "path";
import * as fs from "node:fs"

const walletPath = path.join(__dirname,"./wallet", "/turbin3-wallet.json");
const wallet = JSON.parse(fs.readFileSync(walletPath, "utf8"));
// Import our keypair from the wallet file
// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

const imageUrl="https://raw.githubusercontent.com/Devansh-Aage/SPL-token/refs/heads/main/solana.jpg";

(async () => {
    try {
        //1. Load image
         const response = await fetch(imageUrl);
    const arrayBuffer = await response.arrayBuffer(); // Use arrayBuffer
    const imageBuffer = Buffer.from(arrayBuffer); // Convert to Node.js Buffer

        // Create a generic file from the image buffer
               const genericFile = createGenericFile( imageBuffer, 'image/jpg');

        // Upload the image using Irys uploader
        const [myUri] = await umi.uploader.upload([genericFile]);

        console.log("Your image URI:", myUri);

    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
