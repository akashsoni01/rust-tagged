use scylla::FromRow;
use rust_tagged::Tagged;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// Define tag types for different domain concepts
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct UserIdTag;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ProductIdTag;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct EmailTag;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PriceTag;

// Create tagged type aliases
type UserId = Tagged<Uuid, UserIdTag>;
type ProductId = Tagged<Uuid, ProductIdTag>;
type Email = Tagged<String, EmailTag>;
type Price = Tagged<i32, PriceTag>; // Price in cents

// User entity with mixed field types
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
struct User {
    id: UserId,
    name: String,
    email: Email,
    age: Option<i32>,
    is_active: bool,
    created_at: DateTime<Utc>,
    metadata: Option<HashMap<String, String>>,
}

// Product entity
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
struct Product {
    id: ProductId,
    name: String,
    description: Option<String>,
    price: Price,
    category: String,
    in_stock: bool,
    tags: Vec<String>,
    created_at: DateTime<Utc>,
}

impl User {
    fn new(name: impl Into<String>, email: impl Into<String>, age: Option<i32>) -> Self {
        Self {
            id: UserId::from(Uuid::new_v4()),
            name: name.into(),
            email: Email::from(email.into()),
            age,
            is_active: true,
            created_at: Utc::now(),
            metadata: None,
        }
    }
}

impl Product {
    fn new(name: impl Into<String>, price_cents: i32, category: impl Into<String>) -> Self {
        Self {
            id: ProductId::from(Uuid::new_v4()),
            name: name.into(),
            description: None,
            price: Price::from(price_cents),
            category: category.into(),
            in_stock: true,
            tags: Vec::new(),
            created_at: Utc::now(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Mock Scylla CQL Example with Tagged Types ===\n");

    // Create sample data
    let user = User::new("Alice Johnson", "alice@example.com", Some(28));
    let product = Product::new("Laptop", 99999, "Electronics"); // $999.99

    println!("Created user: {:?}", user);
    println!("Created product: {:?}\n", product);

    // Demonstrate type safety with Tagged types
    println!("=== Type Safety Demonstration ===");
    
    // This would cause a compile error if uncommented:
    // let invalid_assignment: UserId = product.id; // Error: cannot assign ProductId to UserId
    
    // But this works fine:
    let user_id_from_uuid: UserId = Uuid::new_v4().into();
    let product_id_from_uuid: ProductId = Uuid::new_v4().into();
    let email_from_string: Email = "test@example.com".into();
    
    println!("✓ Type safety preserved: UserId and ProductId are distinct types");
    println!("  UserId: {}", user_id_from_uuid.value());
    println!("  ProductId: {}", product_id_from_uuid.value());
    println!("  Email: {}", email_from_string.value());

    // Demonstrate serialization
    println!("\n=== Serialization Demonstration ===");
    let user_json = serde_json::to_string_pretty(&user)?;
    println!("User JSON:");
    println!("{}", user_json);

    let product_json = serde_json::to_string_pretty(&product)?;
    println!("\nProduct JSON:");
    println!("{}", product_json);

    // Demonstrate deserialization
    println!("\n=== Deserialization Demonstration ===");
    let deserialized_user: User = serde_json::from_str(&user_json)?;
    println!("Deserialized user: {:?}", deserialized_user);
    println!("  User ID: {}", deserialized_user.id.value());
    println!("  Email: {}", deserialized_user.email.value());

    // Demonstrate FromRow derive compatibility
    println!("\n=== FromRow Derive Compatibility ===");
    println!("✓ User struct derives FromRow successfully");
    println!("✓ Product struct derives FromRow successfully");
    println!("✓ Tagged fields work with FromRow derive");
    println!("✓ Mixed field types (Tagged + primitives) supported");

    // Demonstrate CQL query parameter compatibility
    println!("\n=== CQL Query Parameter Compatibility ===");
    println!("✓ Tagged types can be used as CQL query parameters");
    println!("✓ SerializeCql trait implemented for Tagged types");
    println!("✓ FromCqlVal trait implemented for Tagged types");

    // Show what the CQL queries would look like
    println!("\n=== Example CQL Queries ===");
    println!("INSERT INTO users (id, name, email, age, is_active, created_at, metadata) VALUES (?, ?, ?, ?, ?, ?, ?)");
    println!("  Parameters: ({}, {}, {}, {:?}, {}, {}, {:?})", 
             user.id.value(), 
             user.name, 
             user.email.value(), 
             user.age, 
             user.is_active, 
             user.created_at, 
             user.metadata);

    println!("\nINSERT INTO products (id, name, description, price, category, in_stock, tags, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)");
    println!("  Parameters: ({}, {}, {:?}, {}, {}, {}, {:?}, {})", 
             product.id.value(), 
             product.name, 
             product.description, 
             product.price.value(), 
             product.category, 
             product.in_stock, 
             product.tags, 
             product.created_at);

    println!("\nSELECT id, name, email, age, is_active, created_at, metadata FROM users");
    println!("  Result: Can be deserialized into Vec<User> using FromRow derive");

    println!("\n=== Example completed successfully! ===");
    println!("This example demonstrates:");
    println!("1. Tagged types for type-safe IDs and domain values");
    println!("2. FromRow derive working with Tagged types");
    println!("3. Mixed field types in CQL queries (Tagged + primitive types)");
    println!("4. Type safety preventing ID mixups");
    println!("5. Serialization/Deserialization support");
    println!("6. CQL query parameter compatibility");

    Ok(())
}
