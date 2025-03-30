# Is there NFT standard in solana?



Yes, Solana has its own standards for Non-Fungible Tokens (NFTs). Unlike Ethereum, which primarily uses ERC-721 and ERC-1155 standards for NFTs, Solana employs a different approach due to its high-performance blockchain architecture. On Solana, NFTs are typically implemented using the **SPL (Solana Program Library) Token standard**, which is similar to Ethereum's ERC-20 but adapted for Solana's ecosystem.

### Key Details About NFTs on Solana:

1. **SPL Token Standard**:

    * The SPL Token standard is used for both fungible and non-fungible tokens on Solana.

    * For NFTs, a token is created with a **supply of exactly 1  and 0 decimals**, ensuring its uniqueness.

    * Metadata (e.g., name, description, image URI) is usually stored off-chain, often using a decentralized storage solution like Arweave or IPFS, and linked to the token via a URI.

2. **Metaplex Protocol:**

    * The most widely adopted framework for NFTs on Solana is the **Metaplex Protocol**.

    * Metaplex provides a standard for creating, managing, and trading NFTs, including a metadata structure that defines attributes like the creator, name, symbol, and media links.

    * It includes programs like the **Token Metadata Program**, which attaches additional data to SPL tokens to make them NFTs.

3. **How It Works:**

    * An NFT on Solana is essentially an SPL token with a unique mint address and a total supply of 1.

    * The metadata is stored in a separate account linked to the token, managed by the Metaplex Token Metadata program.

    * This allows Solana NFTs to include rich attributes, similar to Ethereum NFTs, while benefiting from Solana's high throughput and low transaction costs.

4. **Advantages:**

    * Solana's fast transaction speeds and low fees make it a popular choice for NFT projects, especially for large collections or marketplaces.

    * Projects like **Solana Monkey Business, Degenerate Ape Academy**, and platforms like **Magic Eden** (a leading Solana NFT marketplace) showcase the ecosystem's NFT capabilities.


### Design
This repository provides a basic NFT implementation in Solana using **Anchor**, a development framework for building secure Solana programs. It follows the SPL Token Standard. Its decimals is hardcoded in 0. During the minting process we just specifiy 1 as the amount. After that we will immeditately set the **mint authority** to None to make sure nobody can mint it again. So the supply is guaranteed to be 1.
