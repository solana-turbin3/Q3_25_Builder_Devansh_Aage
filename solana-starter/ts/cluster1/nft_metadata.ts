
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import path from "path";
import * as fs from "node:fs"

const walletPath = path.join(__dirname,"./wallet", "/turbin3-wallet.json");
const wallet = JSON.parse(fs.readFileSync(walletPath, "utf8"));
// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = "https://devnet.irys.xyz/3yWbkq5zrgXVEiNYUoHEoNtnTS6tPGkcNrVHyPUSddqR"
        const metadata = {
            name: "Solana RUG",
            symbol: "SOL",
            description: "Solana RUG made by DEV",
            image: image,
            attributes: [
                {trait_type: "anza", value: '1'}
            ],
            properties: {
                files: [
                    {
                        type: "image/jpg",
                        uri: image
                    },
                ]
            },
            creators: []
        };
        const myUri =await umi.uploader.uploadJson(metadata)
        console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
