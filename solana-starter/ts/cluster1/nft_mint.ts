import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";


import base58 from "bs58";
import path from "path";
import * as fs from "node:fs"

const walletPath = path.join(__dirname,"./wallet", "/turbin3-wallet.json");
const wallet = JSON.parse(fs.readFileSync(walletPath, "utf8"));

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

(async () => {
    let tx = createNft(umi,{
mint,
name:"SOLANA RUG",
sellerFeeBasisPoints:percentAmount(5),
symbol:"DEV",
uri:"https://devnet.irys.xyz/53TagaDsVXNB2kAjCyEn2gUqCaHvTZizvZcpXv14PfV7"
    })
    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);
    
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);
})();