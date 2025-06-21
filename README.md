# 🦀 RustProject – Expression Evaluator in Rust

This is a Rust-based binary expression evaluator that supports constant and nested expressions with `+` and `-` operations. It evaluates expressions like:

(12.000000 + 3.000000)
((10.000000 + 2.000000) + ((1.000000 + 1.000000) + (2.000000 - 1.000000)))

All of these will evaluate correctly, and also be serialized as properly parenthesized strings.

---

## 🔧 Features

Evaluate mathematical expressions.
Serialize expressions into string format with full bracket structure.
Handles deeply nested binary expressions. 
Custom error type for overflow.
Async evaluation support using `tokio`.

---

## 📁 Folder Structure

MyRustProject/
├── src/
│ ├── main.rs # Test harness (provided)
│ └── expression.rs # Your main logic
├── Cargo.toml # Project manifest
└── README.md # This file

---

### Prerequisites
- 🦀 [Install Rust](https://www.rust-lang.org/tools/install)
- 📦 Install `cargo` (comes with Rust)


💡 Built With:
🦀Rust
🦀Tokio – async runtime
