# Rust LeetCode Template

A comprehensive Rust template for solving LeetCode problems with optimized project structure and tooling.

## Features

- 🚀 Pre-configured Cargo.toml with performance optimizations
- 📁 Organized directory structure for Chinese and English problems
- 🔧 Common data structures (TreeNode, ListNode, Node)
- ⚡ Benchmark support with Criterion
- 🧪 Testing framework setup
- 📦 Useful dependencies for algorithm problems

## Project Structure

```
rs-template/
├── Cargo.toml              # Project configuration
├── benches/                # Benchmark tests
│   └── lib.rs
├── leetcode/
│   └── editor/
│       ├── common/         # Common data structures
│       │   ├── mod.rs
│       │   ├── TreeNode.rs
│       │   ├── ListNode.rs
│       │   └── Node.rs
│       ├── cn/             # Chinese version problems
│       └── en/             # English version problems
└── src/                    # Additional source files
```

## Usage

### Running a specific problem
```bash
# Run Chinese version of two sum
cargo run --bin cn_two_sum

# Run English version of binary tree preorder traversal  
cargo run --bin en_binary_tree_preorder_traversal
```

### Running benchmarks
```bash
cargo bench
```

### Running tests
```bash
cargo test
```

## Adding New Problems

1. Create a new `.rs` file in either `leetcode/editor/cn/` or `leetcode/editor/en/`
2. Add a corresponding `[[bin]]` entry in `Cargo.toml`
3. Use the common data structures from `leetcode/editor/common/`

## Dependencies

- `rand`: Random number generation for testing
- `itertools`: Additional iterator methods
- `criterion`: Benchmarking framework (dev dependency)

## Performance Profiles

- **Release**: Maximum optimization with LTO enabled
- **Dev**: Debug-friendly with overflow checks

## License

MIT License