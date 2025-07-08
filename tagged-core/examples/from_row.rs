use scylla::{FromRow, Session, SessionBuilder};
use scylla::transport::session::IntoTypedRows;
use tagged_core::Tagged;
use std::error::Error;

#[derive(Debug, FromRow)]
struct UserRow {
    id: Tagged<i32, Self>,
    // name: Option<Tagged<String, Self>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to Scylla cluster
    let session: Session = SessionBuilder::new()
        .known_node("127.0.0.1:9042")
        .build()
        .await?;

    // Create keyspace and table if not exist
    session
        .query(
            "CREATE KEYSPACE IF NOT EXISTS demo WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1}",
            &[],
        )
        .await?;

    session
        .query(
            "CREATE TABLE IF NOT EXISTS demo.users (id int PRIMARY KEY, name text)",
            &[],
        )
        .await?;

    // Insert data
    let id = Tagged::<i32, UserRow>::new(1);
    let name = Some(Tagged::<String, UserRow>::new("Alice".to_string()));

    session
        .query("INSERT INTO demo.users (id, name) VALUES (?, ?)", (id, name))
        .await?;

    // Select and deserialize
    let rows = session
        .query("SELECT id, name FROM demo.users", &[])
        .await?
        .rows
        .ok_or("No rows returned")?;

    for row in rows.into_typed::<UserRow>() {
        let user = row?;
        println!("User: {:?}", user);
    }

    Ok(())
}
