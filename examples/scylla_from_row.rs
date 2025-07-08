use scylla::{Session, SessionBuilder};
use scylla::transport::errors::QueryError;
use rust_tagged::Tagged;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug)]
struct UserTag;
type UserId = Tagged<UserTag, Uuid>;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: UserId,
    name: String,
    email: String,
}

impl User {
    fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
        User {
            id: UserId::from(Uuid::new_v4()),
            name: name.into(),
            email: email.into(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), QueryError> {
    // Connect to Scylla
    let session: Session = SessionBuilder::new()
        .known_node("127.0.0.1:9042")
        .build()
        .await?;

    // Ensure table exists
    session
        .query(
            "CREATE TABLE IF NOT EXISTS testks.users (
                id UUID PRIMARY KEY,
                name TEXT,
                email TEXT
            )",
            &[],
        )
        .await?;

    let user = User::new("Akash Soni", "akash@example.com");

    // Insert user
    session
        .query(
            "INSERT INTO testks.users (id, name, email) VALUES (?, ?, ?)",
            (user.id.get(), &user.name, &user.email),
        )
        .await?;

    println!("Inserted user: {:?}", user);

    // Read back user
    let rows = session
        .query("SELECT id, name, email FROM testks.users WHERE id = ?", (user.id.get(),))
        .await?
        .rows
        .ok_or_else(|| QueryError::Protocol("No rows returned".into()))?;

    for row in rows {
        let (id, name, email): (Uuid, String, String) = row.into_typed()?;
        let fetched_user = User {
            id: UserId::from(id),
            name,
            email,
        };
        println!("Fetched from DB: {:?}", fetched_user);
    }

    Ok(())
}
