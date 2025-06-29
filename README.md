# rust-tagged

> A lightweight, extensible system for creating type-safe IDs, email addresses, and domain-specific values using Rust's type system.

`rust-tagged` provides a simple way to define strongly typed wrappers over primitive types like `String`, `i32`, `Uuid`, `chrono::DateTime`, etc. It helps eliminate bugs caused by misusing raw primitives for conceptually distinct fields such as `UserId`, `Email`, `ProductId`, and more.

## 🧠 Why Use Tagged Types?

* Eliminate accidental mixups between similar types (e.g. `OrgId` vs `UserId`)
* Enforce domain modeling in code via the type system
* Ergonomic `.into()` support for primitive conversions
* Optional serde and macro support for clean `#[derive(Tagged)]`

---

## ✨ Features

* Lightweight `Tagged<T, Tag>` abstraction
* `From<T>` and `Into<T>` implementations for easy use
* Optional `Deref`, `Display`, `Serialize`, and `Deserialize` support
* Custom derive macro `#[derive(Tagged)]`

---

## 🛠 Installation

```toml
[dependencies]
rust-tagged = "0.1"
```

To enable serde support:

```toml
[dependencies.rust-tagged]
version = "0.1"
features = ["serde"]
```

---

## 🔰 Getting Started (Easy)

```rust
use rust_tagged::*;
use rust_tagged_macros::Tagged;

#[derive(Tagged)]
pub struct UserId(Tagged<i32, User>);

#[derive(Tagged)]
pub struct Email(Tagged<String, User>);

fn main() {
    let id: UserId = 42.into();
    let email: Email = "user@example.com".into();

    println!("UserId: {}", id);
    println!("Email: {}", email);
}
```

### ✅ Output

```
UserId: 42
Email: user@example.com
```

---

## 🧱 Medium: Nesting in Domain Models

```rust
use rust_tagged::*;
use rust_tagged_macros::Tagged;
use uuid::Uuid;

#[derive(Tagged)]
pub struct OrgId(Tagged<Uuid, Org>);

#[derive(Tagged)]
pub struct OrgEmail(Tagged<String, Org>);

#[derive(Debug)]
struct Organization {
    id: OrgId,
    email: OrgEmail,
}

fn main() {
    let org = Organization {
        id: Uuid::new_v4().into(),
        email: "contact@company.com".into(),
    };

    println!("Org ID: {}", org.id);
    println!("Org Email: {}", org.email);
}
```

---

## 🔒 Hard: Timestamped Resources with `chrono` + `serde`

```rust
use rust_tagged::*;
use rust_tagged_macros::Tagged;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Tagged, Serialize, Deserialize)]
pub struct CreatedAt(Tagged<DateTime<Utc>, Audit>);

#[derive(Tagged, Serialize, Deserialize)]
pub struct UpdatedAt(Tagged<DateTime<Utc>, Audit>);

#[derive(Serialize, Deserialize, Debug)]
struct BlogPost {
    title: String,
    created: CreatedAt,
    updated: UpdatedAt,
}

fn main() {
    let post = BlogPost {
        title: "Type-Safe Rust APIs".into(),
        created: Utc::now().into(),
        updated: Utc::now().into(),
    };

    let json = serde_json::to_string_pretty(&post).unwrap();
    println!("Serialized: \n{json}");
}
```

---

## 📦 Custom `Tagged<T, Tag>` API

```rust
use rust_tagged::{Tagged, Tag};

#[derive(Debug)]
struct EmailTag;
impl Tag for EmailTag {}

type Email = Tagged<String, EmailTag>;

fn main() {
    let email: Email = "test@example.com".into();
    println!("Email inner value: {}", email.value());

    // Convert back to String
    let raw: String = email.into();
    println!("Raw String: {raw}");
}
```

---

## 🔌 Serde Integration

When enabled via `features = ["serde"]`, tagged types auto-serialize like their inner types.

```rust
#[derive(Tagged, Serialize, Deserialize)]
pub struct UserId(Tagged<i32, User>);

let id = UserId::from(10);
let json = serde_json::to_string(&id)?; // "10"
```

---

---

## 📃 License

Licensed under either of

* Mozilla Public License 2.0

cargo run -p tagged-core --example basic
