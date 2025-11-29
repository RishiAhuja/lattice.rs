# lattice.rs

<p align="center">
  <img src="https://raw.githubusercontent.com/RishiAhuja/lattice.rs/main/assets/benchmark-example.png" alt="Benchmark Example" width="600">
</p>

A collection of fundamental data structures implemented from scratch in Rust for learning and reference.

## Overview

**lattice.rs** is a hands-on exploration of classic data structures using idiomatic Rust patterns. Each implementation emphasizes clarity, type safety, and ownership semantics while maintaining performance characteristics expected from these structures.

## Implemented Data Structures

### Trees
- **Binary Search Tree (BST)** - Generic binary tree with ordered insertion, search, deletion, and tree traversals
- **AVL Tree** _(in progress)_ - Self-balancing binary search tree
- **B-Tree** _(in progress)_ - Multi-way search tree optimized for disk access

## Performance

Run comprehensive benchmarks to compare data structure performance:

```bash
cargo bench
```

View detailed HTML reports with interactive charts at `target/criterion/report/index.html`

## Contributing

Contributions are welcome! To contribute:

1. **Fork the repository**
   ```bash
   # Click the 'Fork' button on GitHub or use:
   gh repo fork rishiahuja/lattice.rs
   ```

2. **Clone your fork**
   ```bash
   git clone https://github.com/your_username/lattice.rs
   cd lattice.rs
   ```

3. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

4. **Make your changes and commit**
   ```bash
   git add .
   git commit -m "Add: your feature description"
   ```

5. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Open a Pull Request** on the original repository

Feel free to:
- Add new data structures
- Improve existing implementations
- Add tests and documentation
- Fix bugs or optimize code

**Built using Rust 🦀**