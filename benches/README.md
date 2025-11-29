# Benchmarks Guide

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark group
cargo bench BST/insert
cargo bench BST/search

# Run benchmarks for specific size
cargo bench -- 1000

# Save baseline for comparison
cargo bench -- --save-baseline my-baseline

# Compare against baseline
cargo bench -- --baseline my-baseline
```