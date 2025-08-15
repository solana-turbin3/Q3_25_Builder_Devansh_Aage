import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Capstone } from "../target/types/capstone";
import {
  getAssociatedTokenAddress,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
const {LAMPORTS_PER_SOL} =anchor.web3;

describe("capstone", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.capstone as Program<Capstone>;
  const provider=anchor.AnchorProvider.env();


  const NAME= "RajHans Residence";
  const SYMBOL= "RAJ";
  const URI= "https://raw.githubusercontent.com/Devansh-Aage/SPL-token/refs/heads/main/master.json";

  const MONTHLY_RENT=new anchor.BN(0.05 * LAMPORTS_PER_SOL);
  const DEPOSIT_AMOUNT=new anchor.BN( 1 * LAMPORTS_PER_SOL);
  const LATE_FEE_PERCENT= 3;
  const MIN_RENTER_SCORE=15;
  const CANCEL_ALLOWED_AFTER=1;
  const CANCEL_PENALTY_PERCENT=7;
  const MONTHS=3;
  const ITEM_NFT_NAME="RajHans Residence #1";
  const ITEM_NFT_SYMBOL="RAJ";
  const ITEM_NFT_URI= "https://raw.githubusercontent.com/Devansh-Aage/SPL-token/refs/heads/main/member_nft.json"

  const landlord=anchor.web3.Keypair.generate();
  const renter=anchor.web3.Keypair.generate();

    let shared: {
    collectionMintPDA: PublicKey,
    editionMintPDA: PublicKey,
    escrowPDA: PublicKey,
    agreementPDA: PublicKey,
  };

  const TOKEN_METADATA_PROGRAM= new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")

  before(async()=>{
    const collectionMintSeeds=[Buffer.from("collection_mint"),landlord.publicKey.toBuffer()]
    const [collectionMint,collectionMintBump]=PublicKey.findProgramAddressSync(collectionMintSeeds,program.programId)

       const editionMintSeeds=[Buffer.from("edition"),collectionMint.toBuffer()]
    const [editionMint,editionMintBump]=PublicKey.findProgramAddressSync(
      editionMintSeeds,
      program.programId
    )

     const escrowSeeds=[Buffer.from("escrow"),editionMint.toBuffer()]
    const [escrow,escrowBump]=PublicKey.findProgramAddressSync(
      escrowSeeds,
      program.programId
    )

    const agreementSeeds=[Buffer.from("agreement"),renter.publicKey.toBuffer(),landlord.publicKey.toBuffer()]
    const [agreement,agreementBump]=PublicKey.findProgramAddressSync(
      agreementSeeds,
      program.programId
    )

    shared={
      collectionMintPDA:collectionMint,
      editionMintPDA:editionMint,
      escrowPDA:escrow,
      agreementPDA:agreement
    }

    const sig= await provider.connection.requestAirdrop(landlord.publicKey,6*anchor.web3.LAMPORTS_PER_SOL)
    await provider.connection.confirmTransaction(sig)

    const renterSig= await provider.connection.requestAirdrop(renter.publicKey,6*anchor.web3.LAMPORTS_PER_SOL)
    await provider.connection.confirmTransaction(renterSig)
  })

  it("mint collection NFT for landlord", async () => {
    // Add your test here.
    const tx = await program.methods.initLandlord(NAME,SYMBOL,URI).accounts({landlord:landlord.publicKey, tokenProgram:TOKEN_PROGRAM_ID}).signers([landlord]).rpc()
    console.log(`Collection NFT transaction at https://explorer.solana.com/tx/${tx}?cluster=custom`);
  });


  it("init escrow and print item nft", async () => {
    const vaultATA=await getAssociatedTokenAddress(shared.editionMintPDA,shared.escrowPDA,true)
    const initEscrowIx = await program.methods.createEscrow(
      MONTHLY_RENT,
      DEPOSIT_AMOUNT,
      LATE_FEE_PERCENT,
      MIN_RENTER_SCORE,
      CANCEL_ALLOWED_AFTER,
      CANCEL_PENALTY_PERCENT,
      MONTHS,
      ITEM_NFT_NAME,
      ITEM_NFT_SYMBOL,
      ITEM_NFT_URI
    ).accountsPartial({landlord:landlord.publicKey,
      collectionMint:shared.collectionMintPDA,
      escrow:shared.escrowPDA,
      editionMint:shared.editionMintPDA,
      vault:vaultATA,
      tokenProgram:TOKEN_PROGRAM_ID}).instruction();

    const computeIx= anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
      units:300000
    });
    const priorityIx=anchor.web3.ComputeBudgetProgram.setComputeUnitPrice({
      microLamports:1
    });

    const blockWithContext=await provider.connection.getLatestBlockhash()

    const initEscrowTx= new anchor.web3.Transaction({
      feePayer:landlord.publicKey,
      recentBlockhash:blockWithContext.blockhash,
    }).add(initEscrowIx).add(computeIx).add(priorityIx);
    
    const signature= await anchor.web3.sendAndConfirmTransaction(provider.connection,initEscrowTx,[landlord],{skipPreflight:true})
    console.log(`Init Escrow transaction https://explorer.solana.com/tx/${signature}?cluster=custom`);
  });

//   it("check if escrow exists",async()=>{
    
// // Add this check right before the closeEscrow call
// try {
//   const escrowAccount = await program.account.escrow.fetch(shared.escrowPDA);
//   console.log("Escrow exists with data:", escrowAccount);
// } catch (error) {
//   console.error("Escrow account not found or invalid!");
//   throw error;
// }
//   })

  // it("close escrow and burn item nft", async () => {
  //   const vaultATA=await getAssociatedTokenAddress(shared.editionMintPDA,shared.escrowPDA,true)

  //   const metadataSeeds=[Buffer.from("metadata"),TOKEN_METADATA_PROGRAM.toBuffer(),shared.editionMintPDA.toBuffer()];
  //   const [metadata,metadataBump]=PublicKey.findProgramAddressSync(metadataSeeds,TOKEN_METADATA_PROGRAM);

  //   const masterEditionSeeds=[Buffer.from("metadata"),TOKEN_METADATA_PROGRAM.toBuffer(),shared.editionMintPDA.toBuffer(),Buffer.from("edition")]
  //   const [masterEdition, masterEditionBump]=PublicKey.findProgramAddressSync(masterEditionSeeds,TOKEN_METADATA_PROGRAM);

  //   const collectionMetadataSeeds=[Buffer.from("metadata"),TOKEN_METADATA_PROGRAM.toBuffer(),shared.collectionMintPDA.toBuffer()];
  //   const [collectionMetadata,collectionMetadataBump]=PublicKey.findProgramAddressSync(collectionMetadataSeeds,TOKEN_METADATA_PROGRAM);

  //   const closeEscrowIx = await program.methods.closeEscrow().accountsStrict({
  //     collectionMint:shared.collectionMintPDA,
  //     editionMint:shared.editionMintPDA,
  //     escrow:shared.escrowPDA,
  //     vault:vaultATA,
  //     tokenProgram:TOKEN_PROGRAM_ID,
  //     landlord:landlord.publicKey,
  //     metadata:metadata,
  //     masterEdition:masterEdition,
  //     collectionMetadata:collectionMetadata,
  //     associatedTokenProgram:ASSOCIATED_TOKEN_PROGRAM_ID,
  //     systemProgram: SYSTEM_PROGRAM_ID,
  //     tokenMetadataProgram:TOKEN_METADATA_PROGRAM,
  //   }).instruction()

  //   const computeIx= anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
  //     units:300000
  //   });
  //   const priorityIx=anchor.web3.ComputeBudgetProgram.setComputeUnitPrice({
  //     microLamports:1
  //   });

  //   const blockWithContext=await provider.connection.getLatestBlockhash()

  //   const closeEscrowTx=new anchor.web3.Transaction({
  //     feePayer:landlord.publicKey,
  //     recentBlockhash:blockWithContext.blockhash,
  //   }).add(closeEscrowIx).add(computeIx).add(priorityIx);

  //   const signature= await anchor.web3.sendAndConfirmTransaction(provider.connection,closeEscrowTx,[landlord],{skipPreflight:true})

  //   console.log(`Close escrow transaction at https://explorer.solana.com/tx/${signature}?cluster=custom`);
  // });

    it("accept escrow and init agreement", async () => {
     
    const vaultATA=await getAssociatedTokenAddress(shared.editionMintPDA,shared.escrowPDA,true)
    const nftATA=await getAssociatedTokenAddress(shared.editionMintPDA, shared.agreementPDA,true)


    const depositSeeds=[Buffer.from("deposit"),shared.agreementPDA.toBuffer()];
    const [depositPDA,depositBumps]=PublicKey.findProgramAddressSync(depositSeeds,program.programId)

    const tx = await program.methods.takeEscrow().accountsPartial({
      renter:renter.publicKey,
      landlord:landlord.publicKey,
      editionMint:shared.editionMintPDA,
      tokenProgram:TOKEN_PROGRAM_ID,
      agreement:shared.agreementPDA,
      depositVault:depositPDA,
      escrow:shared.escrowPDA,
      nftVault:nftATA,
      vault:vaultATA
    }).signers([renter]).rpc()
    console.log(`Init agreement transaction at https://explorer.solana.com/tx/${tx}?cluster=custom`);
  });

});
