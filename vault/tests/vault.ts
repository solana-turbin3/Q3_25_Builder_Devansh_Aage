import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";

describe("vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.vault as Program<Vault>;
  
  const provider=anchor.AnchorProvider.env();
  anchor.setProvider(provider)
  
  const user = anchor.web3.Keypair.generate()
  before(async()=>{
 const sig=await provider.connection.requestAirdrop(user.publicKey,1e9)
  await provider.connection.confirmTransaction(sig,"confirmed")
  })
 
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
       user: user.publicKey,
    }).signers([user]).rpc();
    console.log("Your transaction signature", tx);   
  });

  it("Deposits",async()=>{
    const depositTx=await program.methods.deposit(new anchor.BN(1000)).accounts({user:user.publicKey
    }).signers([user]).rpc()
  console.log("Your transaction signature", depositTx);
  })

   it("Withdraws",async()=>{
    const withdrawsTx=await program.methods.withdraw(new anchor.BN(400)).accounts({user:user.publicKey
    }).signers([user]).rpc()
  console.log("Your transaction signature", withdrawsTx);
  })

  it("Closes vault",async()=>{
    const closeTx=await program.methods.close().accounts({user:user.publicKey
    }).signers([user]).rpc()
  console.log("Your transaction signature", closeTx);
  })

 
});
