# RUST-SPECTRE 🦀👻

![Status](https://img.shields.io/badge/Status-PoC-orange?style=flat-square) ![Language](https://img.shields.io/badge/Rust-WASM-red?style=flat-square)

**Rust-Spectre** is a Proof-of-Concept (PoC) demonstrating browser-based execution using **WebAssembly (WASM)** generated from Rust. It simulates a client-side agent deployment to demonstrate the capabilities of WASM in modern post-exploitation scenarios.

## 🚀 Features

- **High Performance:** Core logic compiled to WASM for near-native speed.
- **Stealth:** Executes within the browser memory context.
- **Interop:** Seamless communication between Rust (Backend Logic) and JavaScript (DOM Manipulation).

## 🛠️ Installation & Usage

### Prerequisites

- Rust & Cargo
- `wasm-pack` (`cargo install wasm-pack`)

### Quick Start

1. **Build the WASM binary:**
   ```bash
   make build
   # or: wasm-pack build --target web
   ```
