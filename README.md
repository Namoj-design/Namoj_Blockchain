# namoj_blockchain

A simple blockchain implementation written in **Rust**, with networking, storage, wallet management, and API support.  
This project is intended for learning how blockchains work under the hood.

---

## 🚀 Introduction
The `namoj_blockchain` project is a minimal blockchain system that demonstrates core blockchain components such as:
- Block & transaction structures
- Proof-of-work mining
- Persistent storage using **Sled**
- Wallets & cryptographic signatures with **ed25519-dalek**
- REST API endpoints with **Warp** & **Tokio**
- Serialization/deserialization with **Serde**

---

## ✨ Features
- Add and mine new blocks  
- Handle transactions between wallets  
- Basic peer-to-peer (P2P) networking skeleton  
- Persistent chain storage  
- Simple REST API for interacting with the chain  

---

## 🛠 Installation

1. Install [Rust](https://www.rust-lang.org/tools/install) and ensure `cargo` is available:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


