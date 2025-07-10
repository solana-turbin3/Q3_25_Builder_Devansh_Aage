import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";

describe("escrow", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider=anchor.AnchorProvider.env();
  anchor.setProvider(provider)
  const program = anchor.workspace.escrow as Program<Escrow>;

  const MINT_A= new anchor.web3.PublicKey("mnttppYwBzC49BRZ5khBEDBXaJeydya6eGGD9MeXFcy")

  const MINT_B= new anchor.web3.PublicKey("mntVctmZVwmgp5cGFo5bfdYnwpG7ALryUVVGWramWNQ")

  const TOKEN_PROGRAM= new anchor.web3.PublicKey("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb")

  const seed=new anchor.BN(1)
  const receive=new anchor.BN(1e9)
  const deposit=new anchor.BN(15e9)

  const user=anchor.web3.Keypair.generate();
  before(async()=>{
    const sig= await provider.connection.requestAirdrop(user.publicKey,2*anchor.web3.LAMPORTS_PER_SOL)
    await provider.connection.confirmTransaction(sig)
  })


  it("initialize escrow!", async () => {
    // Add your test here.
    const tx = await program.methods.initEscrow(seed,receive).accounts({
      maker:user.publicKey,
      mintA:MINT_A,
      mintB:MINT_B,
      tokenProgram:TOKEN_PROGRAM
    }).signers([user]).rpc();
    console.log("Initialization transaction signature: ", tx);
  });

  it("deposit token in vault",async()=>{
     const tx = await program.methods.deposit(deposit).accounts({
      maker:user.publicKey,
      mintA:MINT_A,
      mintB:MINT_B,
      tokenProgram:TOKEN_PROGRAM
    }).signers([user]).rpc();
    console.log("\nDeposit transaction signature: ", tx);
  })
});
