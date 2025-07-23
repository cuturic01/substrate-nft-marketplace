# Substrate NFT Marketplace

This repository is a **learning project** built during the **PBA‑X** program, following the [Substrate Kitties tutorial](https://dotcodeschool.com/courses/substrate-kitties).  
It demonstrates how to build a simple **NFT marketplace** on a custom blockchain using **Substrate**.

## What it does
- Mint unique Kitty NFTs with generated DNA
- Transfer NFTs between accounts
- List NFTs for sale with a configurable price
- Buy listed NFTs using the chain’s native currency

## Technology Used
- [Substrate](https://substrate.io/) and FRAME, implemented in Rust
- Custom runtime pallet for NFT and marketplace logic
- Wasm runtime environment for executing blockchain logic

## Features
- On‑chain NFT creation and ownership
- Marketplace with pricing, transfers, and purchase functionality
- Events and storage entries updated as extrinsics are executed

## Running Locally
Prerequisites: install Rust (nightly) and add the WebAssembly target.

```bash
git clone https://github.com/cuturic01/substrate-nft-marketplace
cd substrate-nft-marketplace
rustup default nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo build --release
./target/release/node-template --dev
```

## Things learned
- How to create a custom FRAME pallet and integrate it into a Substrate runtime
- How to design storage and events for NFTs and marketplace logic
- How to handle ownership, pricing, and purchase flows on-chain
- How to build and run a local Substrate development node
