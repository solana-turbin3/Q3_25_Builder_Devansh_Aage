import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Capstone } from "../target/types/capstone";
import {
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("capstone", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.capstone as Program<Capstone>;
  const provider=anchor.AnchorProvider.env();


  const NAME= "RajHans Residence";
  const SYMBOL= "RAJ";
  const URI= "https://raw.githubusercontent.com/Devansh-Aage/SPL-token/refs/heads/main/master.json"

  const user=anchor.web3.Keypair.generate();
  before(async()=>{
    const sig= await provider.connection.requestAirdrop(user.publicKey,2*anchor.web3.LAMPORTS_PER_SOL)
    await provider.connection.confirmTransaction(sig)
  })
  it("mint printable NFT", async () => {
    // Add your test here.
    const tx = await program.methods.initLandlord(NAME,SYMBOL,URI).accounts({landlord:user.publicKey, tokenProgram:TOKEN_PROGRAM_ID}).signers([user]).rpc()
    console.log("Printable NFT transaction signature", tx);
  });
});
