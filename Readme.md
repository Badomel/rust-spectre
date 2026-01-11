# RUST-SPECTRE 🦀👻

![Status](https://img.shields.io/badge/Status-PoC-orange?style=flat-square) ![Language](https://img.shields.io/badge/Rust-WASM-red?style=flat-square) ![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)

**Rust-Spectre** is a Proof-of-Concept (PoC) demonstrating browser-based execution using **WebAssembly (WASM)** generated from Rust. It simulates a client-side agent deployment to demonstrate the capabilities of WASM in modern post-exploitation scenarios.

![Demo Preview](https://via.placeholder.com/800x400.png?text=Paste+Your+Terminal+Screenshot+Here)

## 🚀 Features

- **High Performance:** Core logic compiled to WASM for near-native speed.
- **Stealth:** Executes within the browser memory context (Client-side).
- **Interop:** Seamless communication between Rust (Backend Logic) and JavaScript (DOM Manipulation).
- **Cinematic UI:** Terminal-style interface with real-time logging simulation.

## 🛠️ Installation & Usage

### Prerequisites

- Rust & Cargo (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- `wasm-pack` (`cargo install wasm-pack`)
- Python 3 (for local server)

### Quick Start

1. **Build the WASM binary:**
   ```bash
   make build
   # or: wasm-pack build --target web
2. **Start the Listener (Local Server):**
   ```bash
   make serve
   # or: python3 -m http.server 8080
3. **Simulate Attack: Open your browser and navigate to:http://localhost:8080/www/**

***Legal Disclaimer***
**FOR EDUCATIONAL AND RESEARCH PURPOSES ONLY.**

This tool is designed for security research and Red Team simulations to test browser security boundaries. The author is not responsible for any misuse of this code. Always ensure you have explicit permission before testing on any system. This project does not contain actual malicious payloads.
