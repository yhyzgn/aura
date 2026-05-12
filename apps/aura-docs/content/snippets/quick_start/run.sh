#!/usr/bin/env bash
# Run Aura's native component gallery.
cargo run -p aura-gallery

# Run the standalone native documentation application.
cargo run -p aura-docs

# Check both main applications without launching windows.
cargo check -p aura-gallery -p aura-docs
