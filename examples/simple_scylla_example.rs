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

// Simple user entity with Tagged fields
#[derive(Debug, Clone, FromRow)]
struct User {
    id: UserId,
    name: String,
    email: Email,
    age: Option<i32>,
}

impl User {
    fn new(name: impl Into<String>, email: impl Into<String>, age: Option<i32>) -> Self {
        Self {
            id: UserId::from(Uuid::new_v4()),
            name: name.into(),
            email: Email::from(email.into()),
            age,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Scylla cluster
    let session: Session = SessionBuilder::new()
        .known_node("127.0.0.1:9042")
        .build()
        .await?;

    // Create keyspace and table
    session
        .query(
            "CREATE KEYSPACE IF NOT EXISTS demo WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1}",
            &[],
        )
        .await?;

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

    println!("=== Simple Scylla CQL Example with Tagged Types ===\n");

    // Create sample user
    let user = User::new("Alice Johnson", "alice@example.com", Some(28));
    println!("Created user: {:?}", user);

    // Insert user
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

    println!("✓ Inserted user");

    // Query users using FromRow derive
    println!("\n=== Querying Users ===");
    let user_rows = session
        .query("SELECT id, name, email, age FROM demo.users", &[])
        .await?
        .rows
        .ok_or("No rows returned")?;

    for row in user_rows.into_typed::<User>() {
        let fetched_user = row?;
        println!("Fetched user: {:?}", fetched_user);
        println!("  User ID: {}", fetched_user.id.value());
        println!("  Email: {}", fetched_user.email.value());
        println!("  Age: {:?}", fetched_user.age);
        println!();
    }

    // Demonstrate type safety
    println!("=== Type Safety Demonstration ===");
    
    // This would cause a compile error if uncommented:
    // let invalid_assignment: UserId = user.email; // Error: cannot assign Email to UserId
    
    // But this works fine:
    let user_id_from_uuid: UserId = Uuid::new_v4().into();
    let email_from_string: Email = "test@example.com".into();
    
    println!("✓ Type safety preserved: UserId and Email are distinct types");
    println!("  UserId: {}", user_id_from_uuid.value());
    println!("  Email: {}", email_from_string.value());

    println!("\n=== Example completed successfully! ===");
    println!("This example demonstrates:");
    println!("1. Tagged types for type-safe IDs and domain values");
    println!("2. FromRow derive working with Tagged types");
    println!("3. Mixed field types in CQL queries (Tagged + primitive types)");
    println!("4. Type safety preventing ID mixups");

    Ok(())
}
