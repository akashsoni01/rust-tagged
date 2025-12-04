use rust_tagged::{Tagged, Taggable};
use serde::{Serialize, Deserialize};
// cargo check --example json_deserialize_example --features serde

// Define tag types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct UserTag;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct OrderTag;

// Composite key struct (subset of fields)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
struct CompositeKeyStruct {
    id1: u32,
    id2: String,
    id3: String,
}

// Full struct A with composite key and additional fields
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct A {
    id1: u32,
    id2: String,
    id3: String,
    field2: String,
    field3: Option<i32>,
    field4: bool,
}

// Create tagged type aliases
type UserCompositeKey = Tagged<CompositeKeyStruct, UserTag>;
type UserA = Tagged<A, UserTag>;
type OrderCompositeKey = Tagged<CompositeKeyStruct, OrderTag>;
type OrderA = Tagged<A, OrderTag>;

impl A {
    /// Extract composite key from struct A
    fn composite_key(&self) -> CompositeKeyStruct {
        CompositeKeyStruct {
            id1: self.id1,
            id2: self.id2.clone(),
            id3: self.id3.clone(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== JSON Serialization & Deserialization with Composite Keys ===\n");

    // Same JSON string that represents struct A
    let json_string = r#"{
        "id1": 123,
        "id2": "org-456",
        "id3": "dept-789",
        "field2": "Additional Data",
        "field3": 42,
        "field4": true
    }"#;

    println!("=== Same JSON String ===");
    println!("{}", json_string);

    // Example 1: Serialize Tagged<A> to JSON
    println!("\n=== Example 1: Serialize Tagged<A> to JSON ===");
    let struct_a = A {
        id1: 123,
        id2: "org-456".to_string(),
        id3: "dept-789".to_string(),
        field2: "Additional Data".to_string(),
        field3: Some(42),
        field4: true,
    };
    
    let user_a: UserA = Tagged::from(struct_a);
    
    // Serialize using to_json
    let json = user_a.to_json()?;
    println!("  Serialized (compact): {}", json);
    
    // Serialize using to_json_pretty
    let json_pretty = user_a.to_json_pretty()?;
    println!("  Serialized (pretty):");
    println!("{}", json_pretty);

    // Example 2: Serialize Tagged<CompositeKey> to JSON
    println!("\n=== Example 2: Serialize Tagged<CompositeKey> to JSON ===");
    let composite_key = CompositeKeyStruct {
        id1: 123,
        id2: "org-456".to_string(),
        id3: "dept-789".to_string(),
    };
    
    let user_composite_key: UserCompositeKey = Tagged::from(composite_key);
    
    // Serialize using to_json
    let json = user_composite_key.to_json()?;
    println!("  Serialized (compact): {}", json);
    
    // Serialize using to_json_pretty
    let json_pretty = user_composite_key.to_json_pretty()?;
    println!("  Serialized (pretty):");
    println!("{}", json_pretty);

    // Example 3: Deserialize same JSON into Tagged<A>
    println!("\n=== Example 3: Deserialize JSON into Tagged<A> ===");
    let user_a_deserialized: UserA = Tagged::from_json(json_string)?;
    
    println!("  Deserialized UserA:");
    println!("    id1: {}", user_a_deserialized.id1);
    println!("    id2: {}", user_a_deserialized.id2);
    println!("    id3: {}", user_a_deserialized.id3);
    println!("    field2: {}", user_a_deserialized.field2);
    println!("    field3: {:?}", user_a_deserialized.field3);
    println!("    field4: {}", user_a_deserialized.field4);
    
    assert_eq!(user_a.id1, user_a_deserialized.id1);
    assert_eq!(user_a.id2, user_a_deserialized.id2);
    assert_eq!(user_a.id3, user_a_deserialized.id3);
    println!("  ✓ Deserialization successful!");

    // Example 4: Deserialize same JSON into Tagged<CompositeKey>
    println!("\n=== Example 4: Deserialize Same JSON into Tagged<CompositeKey> ===");
    let user_composite_key_deserialized: UserCompositeKey = Tagged::from_json(json_string)?;
    
    println!("  Deserialized UserCompositeKey:");
    println!("    id1: {}", user_composite_key_deserialized.id1);
    println!("    id2: {}", user_composite_key_deserialized.id2);
    println!("    id3: {}", user_composite_key_deserialized.id3);
    
    // Verify the composite key matches
    let extracted_key = user_a_deserialized.composite_key();
    assert_eq!(*user_composite_key_deserialized, extracted_key);
    println!("  ✓ Composite key matches the extracted key from struct A");

    // Example 5: Round-trip Serialization (Serialize -> Deserialize)
    println!("\n=== Example 5: Round-trip Serialization ===");
    
    // Start with Tagged<A>
    let original_a: UserA = Tagged::from(A {
        id1: 999,
        id2: "org-999".to_string(),
        id3: "dept-999".to_string(),
        field2: "Round Trip Test".to_string(),
        field3: Some(100),
        field4: false,
    });
    
    // Serialize
    let json = original_a.to_json()?;
    println!("  Serialized: {}", json);
    
    // Deserialize back
    let round_trip_a: UserA = Tagged::from_json(&json)?;
    
    assert_eq!(original_a.id1, round_trip_a.id1);
    assert_eq!(original_a.id2, round_trip_a.id2);
    assert_eq!(original_a.id3, round_trip_a.id3);
    assert_eq!(original_a.field2, round_trip_a.field2);
    assert_eq!(original_a.field3, round_trip_a.field3);
    assert_eq!(original_a.field4, round_trip_a.field4);
    
    println!("  ✓ Round-trip successful! All fields match");

    // Example 6: Round-trip with CompositeKey
    println!("\n=== Example 6: Round-trip with CompositeKey ===");
    
    let original_key: UserCompositeKey = Tagged::from(CompositeKeyStruct {
        id1: 555,
        id2: "org-555".to_string(),
        id3: "dept-555".to_string(),
    });
    
    // Serialize
    let json = original_key.to_json()?;
    println!("  Serialized: {}", json);
    
    // Deserialize back
    let round_trip_key: UserCompositeKey = Tagged::from_json(&json)?;
    
    assert_eq!(original_key.id1, round_trip_key.id1);
    assert_eq!(original_key.id2, round_trip_key.id2);
    assert_eq!(original_key.id3, round_trip_key.id3);
    
    println!("  ✓ Round-trip successful! All composite key fields match");

    // Example 7: Using from_json_string
    println!("\n=== Example 7: Using from_json_string ===");
    let json_string_owned = json_string.to_string();
    
    let user_a_from_string: UserA = Tagged::from_json_string(json_string_owned.clone())?;
    let composite_key_from_string: UserCompositeKey = Tagged::from_json_string(json_string_owned)?;
    
    println!("  Deserialized from String:");
    println!("    UserA id1: {}", user_a_from_string.id1);
    println!("    CompositeKey id1: {}", composite_key_from_string.id1);
    assert_eq!(user_a_deserialized.id1, user_a_from_string.id1);
    assert_eq!(user_composite_key_deserialized.id1, composite_key_from_string.id1);
    println!("  ✓ Results match deserialization from &str");

    // Example 8: Type safety with different tags
    println!("\n=== Example 8: Type Safety with Different Tags ===");
    
    // Deserialize into OrderTag types
    let order_a: OrderA = Tagged::from_json(json_string)?;
    let order_composite_key: OrderCompositeKey = Tagged::from_json(json_string)?;
    
    println!("  UserA id1: {} (type: {})", user_a_deserialized.id1, user_a_deserialized.type_name());
    println!("  OrderA id1: {} (type: {})", order_a.id1, order_a.type_name());
    println!("  UserCompositeKey id1: {} (type: {})", user_composite_key_deserialized.id1, user_composite_key_deserialized.type_name());
    println!("  OrderCompositeKey id1: {} (type: {})", order_composite_key.id1, order_composite_key.type_name());
    
    // Type safety: These are different types even with same data
    println!("  ✓ UserA and OrderA are distinct types");
    println!("  ✓ UserCompositeKey and OrderCompositeKey are distinct types");
    
    // This would cause a compile error if uncommented:
    // let invalid: OrderA = user_a_deserialized; // Error: cannot assign UserA to OrderA
    // let invalid: OrderCompositeKey = user_composite_key_deserialized; // Error: type mismatch

    // Example 9: Extract composite key from struct A
    println!("\n=== Example 9: Extract Composite Key from Struct A ===");
    let extracted_composite_key = user_a_deserialized.composite_key();
    let tagged_extracted: UserCompositeKey = Tagged::from(extracted_composite_key);
    
    println!("  Extracted CompositeKey:");
    println!("    id1: {}", tagged_extracted.id1);
    println!("    id2: {}", tagged_extracted.id2);
    println!("    id3: {}", tagged_extracted.id3);
    assert_eq!(user_composite_key_deserialized, tagged_extracted);
    println!("  ✓ Extracted key matches deserialized composite key");

    // Example 10: Error handling
    println!("\n=== Example 10: Error Handling ===");
    let invalid_json = "not valid json";
    match UserA::from_json(invalid_json) {
        Ok(a) => println!("  Success: {:?}", a),
        Err(e) => println!("  Error (expected): {}", e),
    }
    
    match UserCompositeKey::from_json(invalid_json) {
        Ok(key) => println!("  Success: {:?}", key),
        Err(e) => println!("  Error (expected): {}", e),
    }

    println!("\n=== Example completed successfully! ===");
    println!("This example demonstrates:");
    println!("1. Serializing Tagged<A> to JSON (to_json, to_json_pretty)");
    println!("2. Serializing Tagged<CompositeKey> to JSON");
    println!("3. Deserializing same JSON into Tagged<A> and Tagged<CompositeKey>");
    println!("4. CompositeKey is a subset of A's fields");
    println!("5. Round-trip serialization for both types");
    println!("6. Using from_json_string() method");
    println!("7. Type safety with different tags (UserTag vs OrderTag)");
    println!("8. Extracting composite key from full struct");
    println!("9. Error handling for invalid JSON");
    println!("10. Real-world use case: Same JSON for full entity and composite key lookups");

    Ok(())
}
