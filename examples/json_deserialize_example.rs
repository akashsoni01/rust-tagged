use rust_tagged::{Tagged, Taggable};
use serde::Deserialize;

// Define tag types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct UserIdTag;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct EmailTag;

// Create tagged type aliases
type UserId = Tagged<u32, UserIdTag>;
type Email = Tagged<String, EmailTag>;

// Example struct with Tagged fields
#[derive(Debug, Deserialize)]
struct User {
    id: UserId,
    email: Email,
    name: String,
    age: Option<u32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== JSON Deserialization Example ===\n");

    // Example 1: Deserialize a simple Tagged type from JSON
    println!("=== Example 1: Simple Tagged Type ===");
    let json = "42";
    let user_id: UserId = Tagged::from_json(json)?;
    println!("  JSON: {}", json);
    println!("  Deserialized UserId: {}", user_id.value());

    // Example 2: Deserialize a Tagged String from JSON
    println!("\n=== Example 2: Tagged String ===");
    let json = "\"user@example.com\"";
    let email: Email = Tagged::from_json(json)?;
    println!("  JSON: {}", json);
    println!("  Deserialized Email: {}", email.value());

    // Example 3: Deserialize a struct containing Tagged fields
    println!("\n=== Example 3: Struct with Tagged Fields ===");
    let json = r#"{
        "id": 123,
        "email": "alice@example.com",
        "name": "Alice Johnson",
        "age": 28
    }"#;
    
    let user: User = serde_json::from_str(json)?;
    println!("  JSON: {}", json);
    println!("  Deserialized User:");
    println!("    ID: {} (type: {})", user.id.value(), user.id.type_name());
    println!("    Email: {} (type: {})", user.email.value(), user.email.type_name());
    println!("    Name: {}", user.name);
    println!("    Age: {:?}", user.age);

    // Example 4: Using from_json_string method
    println!("\n=== Example 4: Using from_json_string ===");
    let json_string = String::from("999");
    let user_id: UserId = Tagged::from_json_string(json_string)?;
    println!("  Deserialized UserId: {}", user_id.value());

    // Example 5: Error handling
    println!("\n=== Example 5: Error Handling ===");
    let invalid_json = "not a number";
    match Tagged::<u32, UserIdTag>::from_json(invalid_json) {
        Ok(id) => println!("  Success: {}", id.value()),
        Err(e) => println!("  Error (expected): {}", e),
    }

    // Example 6: Type safety demonstration
    println!("\n=== Example 6: Type Safety ===");
    let user_id_json = "42";
    let email_json = "\"test@example.com\"";
    
    let user_id: UserId = Tagged::from_json(user_id_json)?;
    let email: Email = Tagged::from_json(email_json)?;
    
    println!("  UserId: {} (type: {})", user_id.value(), user_id.type_name());
    println!("  Email: {} (type: {})", email.value(), email.type_name());
    
    // This would cause a compile error if uncommented:
    // let invalid: Email = Tagged::from_json(user_id_json)?; // Error: type mismatch
    
    println!("✓ Type safety preserved: UserId and Email are distinct types");

    println!("\n=== Example completed successfully! ===");
    println!("This example demonstrates:");
    println!("1. Deserializing JSON strings into Tagged types");
    println!("2. Using from_json() for &str");
    println!("3. Using from_json_string() for String");
    println!("4. Deserializing structs containing Tagged fields");
    println!("5. Error handling for invalid JSON");
    println!("6. Type safety with Tagged types");

    Ok(())
}

