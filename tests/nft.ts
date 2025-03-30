import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OwnedNft } from "../target/types/owned_nft";
import {
  TOKEN_2022_PROGRAM_ID,
  TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  getMint,
  getAccount,
} from "@solana/spl-token";
const assert = require("assert");

describe("owned_nft", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.OwnedNft as Program<OwnedNft>;

  const alice = anchor.web3.Keypair.generate();
  const bob = anchor.web3.Keypair.generate();
  const peter = anchor.web3.Keypair.generate();

  const [nftInfo] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("nft_info")],
    program.programId
  );

  async function confirmTransaction(tx) {
      const latestBlockHash = await anchor.getProvider().connection.getLatestBlockhash();
      await anchor.getProvider().connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: tx,
      });
  }

  before(async () => {
      const airdrop_alice_tx = await anchor.getProvider().connection.requestAirdrop(alice.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);
      await confirmTransaction(airdrop_alice_tx);
  
      const airdrop_bob_tx = await anchor.getProvider().connection.requestAirdrop(bob.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);
      await confirmTransaction(airdrop_bob_tx);

  });
  

  it("Alice initializes the contract", async () => {

    const tx = await program.methods
      .initialize()
      .accounts({
        signer: alice.publicKey,
      })
      .signers([alice])
      .rpc();
  });


  it("Alice mint nft to bob", async () => {

    const counterAccount = await program.account.nftInfo.fetch(nftInfo);
    const count = counterAccount.count.toNumber();

    const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(new anchor.BN(count).toArray("le", 8))],
      program.programId
    );

    const receiverTokenAccount = await getAssociatedTokenAddress(
      mint,
      bob.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const tx = await program.methods
      .mintNft("Test NFT",
        "TNFT",
        "https://example.com/nft.json")
      .accounts({
        signer: alice.publicKey,
        receiver: bob.publicKey,
        receiverTokenAccount: receiverTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([alice])
      .rpc();

      const [_nftData] = anchor.web3.PublicKey.findProgramAddressSync(
        [mint.toBuffer()],
        program.programId
      );

      const nftData = await program.account.nftData.fetch(_nftData);
      assert.equal(nftData.name, "Test NFT");
      assert.equal(nftData.symbol, "TNFT");
      assert.equal(nftData.uri, "https://example.com/nft.json");
      assert.equal(nftData.minter.toString(), alice.publicKey.toString());
      assert.equal(nftData.currentHolder.toString(), bob.publicKey.toString());

  });


  it("Alice mint nft to peter", async () => {

    const counterAccount = await program.account.nftInfo.fetch(nftInfo);
    const count = counterAccount.count.toNumber();

    assert.equal(count, 1);

    const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(new anchor.BN(count).toArray("le", 8))],
      program.programId
    );

    const receiverTokenAccount = await getAssociatedTokenAddress(
      mint,
      peter.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const tx = await program.methods
      .mintNft("Test NFT 2",
        "TNFT 2",
        "https://example.com/nft2.json")
      .accounts({
        signer: alice.publicKey,
        receiver: peter.publicKey,
        receiverTokenAccount: receiverTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([alice])
      .rpc();

      const [_nftData] = anchor.web3.PublicKey.findProgramAddressSync(
        [mint.toBuffer()],
        program.programId
      );

      const nftData = await program.account.nftData.fetch(_nftData);
      assert.equal(nftData.name, "Test NFT 2");
      assert.equal(nftData.symbol, "TNFT 2");
      assert.equal(nftData.uri, "https://example.com/nft2.json");
      assert.equal(nftData.minter.toString(), alice.publicKey.toString());
      assert.equal(nftData.currentHolder.toString(), peter.publicKey.toString());

  });

  it("Bob transfer nft id 0 to peter", async () => {

    const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(new anchor.BN(0).toArray("le", 8))],
      program.programId
    );

    const fromTokenAccount = await getAssociatedTokenAddress(
      mint,
      bob.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const toTokenAccount = await getAssociatedTokenAddress(
      mint,
      peter.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const tx = await program.methods
      .transferNft()
      .accounts({
        signer: bob.publicKey,
        mint:mint,
        fromTokenAccount: fromTokenAccount,
        toTokenAccount: toTokenAccount,
        to: peter.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([bob])
      .rpc();

      const [_nftData] = anchor.web3.PublicKey.findProgramAddressSync(
        [mint.toBuffer()],
        program.programId
      );

      const nftData = await program.account.nftData.fetch(_nftData);
      assert.equal(nftData.currentHolder.toString(), peter.publicKey.toString());

  });

  
});