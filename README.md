# rust-tagged

> A lightweight, extensible system for creating type-safe IDs, email addresses, and domain-specific values using Rust's type system.

`rust-tagged` provides a simple way to define strongly typed wrappers over primitive types like `String`, `i32`, `Uuid`, `chrono::DateTime`, etc. It helps eliminate bugs caused by misusing raw primitives for conceptually distinct fields such as `UserId`, `Email`, `ProductId`, and more.

## 🧠 Why Use Tagged Types?

* Eliminate accidental mixups between similar types (e.g. `OrgId` vs `UserId`)
* Enforce domain modeling in code via the type system
* Ergonomic `.into()` support for primitive conversions
* Optional serde and macro support for clean `#[derive(Tagged)]`

### 📚 Conceptual References

* [Phantom types (Rust Nomicon)](https://doc.rust-lang.org/nomicon/phantom-data.html)
* [Tagged unions (Wikipedia)](https://en.wikipedia.org/wiki/Tagged_union)
* [Phantom types in Haskell](https://wiki.haskell.org/Phantom_type)
* [Newtype pattern in Rust](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
* [Algebraic data types](https://en.wikipedia.org/wiki/Algebraic_data_type)

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

## 📦 Custom `Tagged<T>` API

```rust
use rust_tagged::{Tagged};

#[derive(Debug)]
struct EmailTag;

type Email = Tagged<String, EmailTag>;

fn main() {
    let email: Email = "test@example.com".into();
    println!("Email inner value: {}", email.value());

    // Convert back to String
    let raw: String = email.into();
    println!("Raw String: {raw}");
}
```

## 🔰 Getting Started (Easy)

```rust
use rust_tagged::Tagged;

struct Employee {
    id: Tagged<i32, Self>,
    employee_email_id: Tagged<String, Self>,
    name: String,
    org: Org,
}

struct Org {
    org_email_id: Tagged<String, Self>,
    name: String,
}

fn send_mail_employee(mail_id: &Tagged<String, crate::Employee>, message: &str) {
    send_mail(mail_id, message);
}

fn send_mail_org(mail_id: &Tagged<String, crate::Org>, message: &str) {
    send_mail(mail_id, message);
}

fn send_mail(mail_id: &str, message: &str) {
    println!("Mail Sent.{}", message);
}


fn main() {
    let emp = Employee {
        id: 12.into(),
        employee_email_id: "akash@gmail.com".into(),
        name: "Akash".into(),
        org: Org {
            org_email_id: "info@codefonsi.com".into(),
            name: "Codefonsi".into(),
        },
    };

    // here we can clearly define and distinct the mail id of employee and org
    // without
    // // expected `&Tagged<String, Org>`, but found `&Tagged<String, Employee>`
    // send_mail_org(&emp.employee_email_id, "This is supposed to send to user but there is no type safety at compile time");
    // 
    // // expected `&Tagged<String, Employee>`, but found `&Tagged<String, Org>`
    // send_mail_employee(&emp.org.org_email_id, "This is supposed to send to user but there is no type safety at compile time");
    //
    // // after refactoring
    // // the trait bound `Tagged<String, Employee>: From<Tagged<String, Org>>` is not satisfied [E0277]
    // send_mail_employee(&emp.org.org_email_id.into(), "This is ok");

    // We don't need review and refactoring the code for runtime mistakes.
    send_mail_org(&emp.org.org_email_id, "This is ok");
    send_mail_employee(&emp.employee_email_id, "This is ok");


}
```

### ✅ Output

```
UserId: 42
Email: user@example.com
```

---

## Coming Soon 
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

## Coming Soon 
## Timestamped Resources with `chrono` + `serde`

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


---

## Coming Soon 
## 🔌 Serde Integration

When enabled via `features = ["serde"]`, tagged types auto-serialize like their inner types.

```rust
#[derive(Tagged, Serialize, Deserialize)]
pub struct UserId(Tagged<i32, User>);

let id = UserId::from(10);
let json = serde_json::to_string(&id)?; // "10"
```

---

## 📃 License

Licensed under either of

* Mozilla Public License 2.0
