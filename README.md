# rust-tagged

Library Design Proposal

# 🏷️ `tagged`: Type-Safe Phantom-Wrapped Raw Values in Rust

> A minimal, zero-cost wrapper around raw values (like `i32`, `String`, etc.) using phantom types for **compile-time safety**.

## ✨ Features

- ✅ Zero-cost abstraction (no runtime overhead)
- ✅ Prevents mixing up incompatible types like `UserId` and `SubscriptionId`
- ✅ `serde` (de)serialization support
- ✅ Implements `Clone`, `Copy`, `Eq`, `PartialEq`, `Ord`, `Hash`, `Debug`, `Display`
- ✅ `From<i32>`, `From<&str>` for ergonomic usage
- ✅ Works seamlessly with `serde_json` and `typealias` style
