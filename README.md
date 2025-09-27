# rust-tagged (v0.5.0)

> A lightweight, extensible system for creating type-safe IDs, email addresses, and domain-specific values using Rust's type system.

`rust-tagged` provides a simple way to define strongly typed wrappers over primitive types like `String`, `i32`, `Uuid`, `chrono::DateTime`, etc. It helps eliminate bugs caused by misusing raw primitives for conceptually distinct fields such as `UserId`, `Email`, `ProductId`, and more.

## 🧠 Why Use Tagged Types?

* Eliminate accidental mixups between similar types (e.g. `OrgId` vs `UserId`)
* Enforce domain modeling in code via the type system
* Ergonomic `.into()` support for primitive conversions
* Optional serde and macro support for clean `#[derive(Tagged)]`
* Scylla CQL integration with `FromRow` derive support

### 📚 Conceptual References

* [Phantom types (Rust Nomicon)](https://doc.rust-lang.org/nomicon/phantom-data.html)
* [Tagged unions (Wikipedia)](https://en.wikipedia.org/wiki/Tagged_union)
* [Phantom types in Haskell](https://wiki.haskell.org/Phantom_type)
* [Newtype pattern in Rust](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
* [Algebraic data types](https://en.wikipedia.org/wiki/Algebraic_data_type)
* [Communicating in Types](https://youtu.be/SOz66dcsuT8?t=1483)

---

## ✨ Features

* Lightweight `Tagged<T, Tag>` abstraction
* `From<T>` and `Into<T>` implementations for easy use
* Optional `Deref`, `Display`, `Serialize`, and `Deserialize` support
* Scylla CQL integration with `FromCqlVal` and `SerializeCql` trait implementations
* `FromRow` derive support for seamless database integration

---

## 🛠 Installation

```toml
[dependencies]
rust-tagged = "_._._"
```

To enable serde support:

```toml
[dependencies.rust-tagged]
version = "_._._"
features = ["full"] # for serde and scylla
```

To enable Scylla CQL support:

```toml
[dependencies.rust-tagged]
version = "_._._"
features = ["scylla"]
```

To enable both serde and Scylla support:

```toml
[dependencies.rust-tagged]
version = "_._._"
features = ["full"]
```
## 🧪 Example - Debug

```rust
use tagged_core::Tagged;

#[derive(Debug)]
struct UserIdTag {
    a: Tagged<u32, Self>,
    b: Tagged<u32, Self>,
}

fn main() {
    let instance = UserIdTag { a: 1.into(), b: 2.into() };
    println!("{}", instance.a);
    println!("{:?}", instance.b);
}
```

---

## 🔐 Example - Hash

```rust
use tagged_core::Tagged;
use std::collections::HashSet;

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
struct User {
    id: Tagged<String, Self>,
}

fn main() {
    let mut s: HashSet<User> = HashSet::new();
    let user = User { id: "me@example.com".into() };
    s.insert(user.clone());

    assert!(s.contains(&user));
}
```

---

## 🔁 Example - Iter

```rust
use tagged_core::Tagged;

#[derive(Debug)]
struct Org;

type EmployeeNames = Tagged<Vec<String>, Org>;

fn main() {
    let names: EmployeeNames = Tagged::new(vec!["Alice".into(), "Bob".into()]);

    for name in &names {
        println!("Name: {name}");
    }

    for name in names {
        println!("Owned: {name}");
    }
}
```

---

## ✏️ Example - Mutation

```rust
use tagged_core::Tagged;

#[derive(Debug)]
struct Org;

type OrgName = Tagged<String, Org>;

fn main() {
    let mut name = OrgName::new("Codefonsi".into());
    name.set("New Org Name".into());

    println!("Updated Org Name: {}", name.value());
}
```

---

## 📦 Custom `Tagged<T>` API

```rust
use tagged_core::Tagged;

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

---

## 🔰 Getting Started (Easy)

```rust
use tagged_core::Tagged;

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

fn send_mail_employee(mail_id: &Tagged<String, Employee>, message: &str) {
    send_mail(mail_id, message);
}

fn send_mail_org(mail_id: &Tagged<String, Org>, message: &str) {
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

    send_mail_org(&emp.org.org_email_id, "This is ok");
    send_mail_employee(&emp.employee_email_id, "This is ok");
}
```

### ✅ Output

```
Mail Sent.This is ok
Mail Sent.This is ok
```

---

## 🧱 Medium: Nesting in Domain Models

```rust
use tagged_core::*;
use uuid::Uuid;

struct Org;

type OrgId = Tagged<Uuid, Org>;
type OrgEmail = Tagged<String, Org>;

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
use tagged_core::*;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

struct Audit;

type CreatedAt = Tagged<DateTime<Utc>, Audit>;
type UpdatedAt = Tagged<DateTime<Utc>, Audit>;

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

## 🗄️ Example - Scylla CQL Integration

```rust
use scylla::{FromRow, Session, SessionBuilder};
use scylla::transport::session::IntoTypedRows;
use rust_tagged::Tagged;
use uuid::Uuid;

// Define tag types for different domain concepts
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct UserIdTag;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct EmailTag;

// Create tagged type aliases
type UserId = Tagged<Uuid, UserIdTag>;
type Email = Tagged<String, EmailTag>;

// User entity with Tagged fields
#[derive(Debug, Clone, FromRow)]
struct User {
    id: UserId,
    name: String,
    email: Email,
    age: Option<i32>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Scylla cluster
    let session: Session = SessionBuilder::new()
        .known_node("127.0.0.1:9042")
        .build()
        .await?;

    // Create table
    session
        .query(
            "CREATE TABLE IF NOT EXISTS demo.users (
                id UUID PRIMARY KEY,
                name TEXT,
                email TEXT,
                age INT
            )",
            &[],
        )
        .await?;

    // Insert user
    let user = User {
        id: UserId::from(Uuid::new_v4()),
        name: "Alice Johnson".to_string(),
        email: Email::from("alice@example.com".to_string()),
        age: Some(28),
    };

    session
        .query(
            "INSERT INTO demo.users (id, name, email, age) VALUES (?, ?, ?, ?)",
            (
                user.id.value(),
                &user.name,
                user.email.value(),
                user.age,
            ),
        )
        .await?;

    // Query users using FromRow derive
    let user_rows = session
        .query("SELECT id, name, email, age FROM demo.users", &[])
        .await?
        .rows
        .ok_or("No rows returned")?;

    for row in user_rows.into_typed::<User>() {
        let fetched_user = row?;
        println!("User: {:?}", fetched_user);
        // Type safety: fetched_user.id is UserId, fetched_user.email is Email
    }

    Ok(())
}
```

This example demonstrates:
- Type-safe database operations with Tagged types
- `FromRow` derive working seamlessly with Tagged fields
- Prevention of ID mixups at compile time
- Clean integration with Scylla CQL queries

---

## 📃 License

Licensed under either of:

* Mozilla Public License 2.0
