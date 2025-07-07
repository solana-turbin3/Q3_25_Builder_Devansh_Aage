import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import * as fs from "node:fs"
import path from "node:path";

const walletPath = path.join(__dirname,"./wallet", "/turbin3-wallet.json");
const wallet = JSON.parse(fs.readFileSync(walletPath, "utf8"));

// Define our Mint address
const mint = publicKey("Cnn74hnQtfATFU41AqjkP8Bpwv58sVE35YsR4Bc7HL4S")

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

(async () => {
    try {
        // Start here
        let accounts: CreateMetadataAccountV3InstructionAccounts = {
            mint,
            mintAuthority:signer,
            payer:signer,

        }

        let data: DataV2Args = {
            sellerFeeBasisPoints:0,
            name:"DevByDevansh",
            symbol:"DEV",
            uri:"https://raw.githubusercontent.com/Devansh-Aage/SPL-token/refs/heads/main/nft2.json",
            creators:null,
            collection:null,
            uses:null
        }

        let args: CreateMetadataAccountV3InstructionArgs = {
            data,
            isMutable:true,
            collectionDetails:null
        }

        let tx = createMetadataAccountV3(
            umi,
            {
                ...accounts,
                ...args
            }
        )

        let result = await tx.sendAndConfirm(umi);
        console.log(bs58.encode(result.signature));
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
