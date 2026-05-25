# Transaction Queue CLI — Phase 0 🦀

A minimal, low-level transaction simulation system built from first principles in Rust. This component serves as the foundational mempool execution layer for a custom distributed state machine architecture.

## 🛠️ Overview

This CLI manages local state transitions by queuing incoming payloads, sorting execution priorities, and preparing raw transactions for block inclusion. No high-level abstractions, just pure systems-level state handling.

### Core Features
* **Memory Pool Management:** High-performance local sequencing queue for incoming raw transactions.
* **Deterministic Execution:** Simulates state validation and ordering before blocks are packed.
* **Zero-Fluff CLI:** Terminal interface designed for raw execution tracking.

## 🚀 Quick Start

Ensure you have the Rust toolchain installed, then spin up the CLI locally:

```bash
# Build the binary
cargo build --release

# Run the simulation engine
cargo run
git rm --cached transaction_queue_cli/.README.md.swp 2>/dev/null
git add transaction_queue_cli/README.md
git commit -m "docs: clean up readme and add phase 0 documentation"
