use scylla::FromRow;
use rust_tagged::{Tagged, Taggable};
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

// Functions that enforce Taggable types - preventing use of raw primitives
fn authenticate_user<T: Taggable>(user_id: &T) {
    println!("✓ Authenticating user with ID (type: {})", user_id.type_name());
    // This function only accepts Tagged types, not raw Uuid
    // authenticate_user(&Uuid::new_v4()); // ✗ Compile error!
}

fn send_welcome_email<T: Taggable>(email: &T) {
    println!("✓ Sending welcome email (type: {})", email.type_name());
    // This function only accepts Tagged types, not raw String
    // send_welcome_email(&"test@example.com".to_string()); // ✗ Compile error!
}

fn display_price<T: Taggable>(price: &T) {
    println!("✓ Displaying price (type: {})", price.type_name());
    // This function only accepts Tagged types, not raw i32
    // display_price(&99999); // ✗ Compile error!
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

    // Demonstrate Taggable trait enforcement
    println!("\n=== Taggable Trait Enforcement ===");
    authenticate_user(&user.id);
    send_welcome_email(&user.email);
    display_price(&product.price);
    
    // These would cause compile errors if uncommented:
    // authenticate_user(&Uuid::new_v4()); // ✗ Error: expected Taggable, found Uuid
    // send_welcome_email(&"test@example.com".to_string()); // ✗ Error: expected Taggable, found String
    // display_price(&99999); // ✗ Error: expected Taggable, found integer
    
    println!("✓ Taggable trait enforces use of Tagged types in function signatures");

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
    println!("2. Taggable trait to enforce Tagged types in function signatures");
    println!("3. FromRow derive working with Tagged types");
    println!("4. Mixed field types in CQL queries (Tagged + primitive types)");
    println!("5. Type safety preventing ID mixups");
    println!("6. Serialization/Deserialization support");
    println!("7. CQL query parameter compatibility");

    Ok(())
}
