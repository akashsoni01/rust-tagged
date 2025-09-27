use scylla::{FromRow, Session, SessionBuilder};
use scylla::transport::session::IntoTypedRows;
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
struct OrderIdTag;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct EmailTag;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PriceTag;

// Create tagged type aliases
type UserId = Tagged<Uuid, UserIdTag>;
type ProductId = Tagged<Uuid, ProductIdTag>;
type OrderId = Tagged<Uuid, OrderIdTag>;
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
    tags: Option<Vec<String>>,
    created_at: DateTime<Utc>,
}

// Order entity with relationships
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
struct Order {
    id: OrderId,
    user_id: UserId,
    product_id: ProductId,
    quantity: i32,
    total_price: Price,
    status: String,
    order_date: DateTime<Utc>,
    notes: Option<String>,
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
            tags: Some(Vec::new()),
            created_at: Utc::now(),
        }
    }
}

impl Order {
    fn new(user_id: UserId, product_id: ProductId, quantity: i32, unit_price: Price) -> Self {
        let total_price = Price::from(unit_price.value() * quantity);
        Self {
            id: OrderId::from(Uuid::new_v4()),
            user_id,
            product_id,
            quantity,
            total_price,
            status: "pending".to_string(),
            order_date: Utc::now(),
            notes: None,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Scylla cluster
    let session: Session = match SessionBuilder::new()
        .known_node("127.0.0.1:9042")
        .build()
        .await
    {
        Ok(session) => session,
        Err(e) => {
            println!("⚠️  Could not connect to Scylla database: {}", e);
            println!("This example requires a running Scylla/Cassandra instance on 127.0.0.1:9042");
            println!("To run this example:");
            println!("1. Start Scylla: docker run --name scylla -p 9042:9042 -d scylladb/scylla");
            println!("2. Wait for it to start: docker exec scylla nodetool status");
            println!("3. Run this example again");
            println!("\nAlternatively, run the mock example that doesn't require a database:");
            println!("cargo run --example mock_scylla_example --features scylla");
            return Ok(());
        }
    };

    // Create keyspace
    session
        .query(
            "CREATE KEYSPACE IF NOT EXISTS ecommerce WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1}",
            &[],
        )
        .await?;

    // Create tables
    session
        .query(
            "CREATE TABLE IF NOT EXISTS ecommerce.users (
                id UUID PRIMARY KEY,
                name TEXT,
                email TEXT,
                age INT,
                is_active BOOLEAN,
                created_at TIMESTAMP,
                metadata MAP<TEXT, TEXT>
            )",
            &[],
        )
        .await?;

    session
        .query(
            "CREATE TABLE IF NOT EXISTS ecommerce.products (
                id UUID PRIMARY KEY,
                name TEXT,
                description TEXT,
                price INT,
                category TEXT,
                in_stock BOOLEAN,
                tags LIST<TEXT>,
                created_at TIMESTAMP
            )",
            &[],
        )
        .await?;

    session
        .query(
            "CREATE TABLE IF NOT EXISTS ecommerce.orders (
                id UUID PRIMARY KEY,
                user_id UUID,
                product_id UUID,
                quantity INT,
                total_price INT,
                status TEXT,
                order_date TIMESTAMP,
                notes TEXT
            )",
            &[],
        )
        .await?;

    println!("=== E-commerce CQL Example with Tagged Types ===\n");

    // Create sample data
    let user = User::new("Alice Johnson", "alice@example.com", Some(28));
    let product = Product::new("Laptop", 99999, "Electronics"); // $999.99
    let order = Order::new(user.id.clone(), product.id.clone(), 1, product.price.clone());

    println!("Created user: {:?}", user);
    println!("Created product: {:?}", product);
    println!("Created order: {:?}\n", order);

    // Insert user
    session
        .query(
            "INSERT INTO ecommerce.users (id, name, email, age, is_active, created_at, metadata) VALUES (?, ?, ?, ?, ?, ?, ?)",
            (
                user.id.value(),
                &user.name,
                user.email.value(),
                user.age,
                user.is_active,
                user.created_at,
                user.metadata,
            ),
        )
        .await?;

    println!("✓ Inserted user");

    // Insert product
    session
        .query(
            "INSERT INTO ecommerce.products (id, name, description, price, category, in_stock, tags, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            (
                product.id.value(),
                &product.name,
                product.description.as_ref(),
                product.price.value(),
                &product.category,
                product.in_stock,
                product.tags.as_ref(),
                product.created_at,
            ),
        )
        .await?;

    println!("✓ Inserted product");

    // Insert order
    session
        .query(
            "INSERT INTO ecommerce.orders (id, user_id, product_id, quantity, total_price, status, order_date, notes) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            (
                order.id.value(),
                order.user_id.value(),
                order.product_id.value(),
                order.quantity,
                order.total_price.value(),
                &order.status,
                order.order_date,
                order.notes.as_ref(),
            ),
        )
        .await?;

    println!("✓ Inserted order\n");

    // Query users using FromRow derive
    println!("=== Querying Users ===");
    let user_rows = session
        .query("SELECT id, name, email, age, is_active, created_at, metadata FROM ecommerce.users", &[])
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

    // Query products using FromRow derive
    println!("=== Querying Products ===");
    let product_rows = session
        .query("SELECT id, name, description, price, category, in_stock, tags, created_at FROM ecommerce.products", &[])
        .await?
        .rows
        .ok_or("No rows returned")?;

    for row in product_rows.into_typed::<Product>() {
        let fetched_product = row?;
        println!("Fetched product: {:?}", fetched_product);
        println!("  Product ID: {}", fetched_product.id.value());
        println!("  Price: ${:.2}", *fetched_product.price.value() as f64 / 100.0);
        println!("  Category: {}", fetched_product.category);
        println!();
    }

    // Query orders using FromRow derive
    println!("=== Querying Orders ===");
    let order_rows = session
        .query("SELECT id, user_id, product_id, quantity, total_price, status, order_date, notes FROM ecommerce.orders", &[])
        .await?
        .rows
        .ok_or("No rows returned")?;

    for row in order_rows.into_typed::<Order>() {
        let fetched_order = row?;
        println!("Fetched order: {:?}", fetched_order);
        println!("  Order ID: {}", fetched_order.id.value());
        println!("  User ID: {}", fetched_order.user_id.value());
        println!("  Product ID: {}", fetched_order.product_id.value());
        println!("  Total: ${:.2}", *fetched_order.total_price.value() as f64 / 100.0);
        println!();
    }

    // Complex query: Get orders with user information
    println!("=== Complex Query: User Orders ===");
    let order_rows = session
        .query(
            "SELECT id, user_id, product_id, quantity, total_price, status, order_date, notes FROM ecommerce.orders",
            &[]
        )
        .await?
        .rows
        .ok_or("No rows returned")?;

    for row in order_rows.into_typed::<Order>() {
        let order = row?;
        
        // Get user information for this order
        let user_rows = session
            .query(
                "SELECT id, name, email, age, is_active, created_at, metadata FROM ecommerce.users WHERE id = ?",
                (order.user_id.value(),)
            )
            .await?
            .rows
            .ok_or("No rows returned")?;

        for user_row in user_rows.into_typed::<User>() {
            let user = user_row?;
            println!("User: {} ({}) - Order: {} - Total: ${:.2} - Status: {}", 
                     user.name, user.email.value(), order.id.value(), 
                     *order.total_price.value() as f64 / 100.0, order.status);
        }
    }

    // Update order status
    println!("\n=== Updating Order Status ===");
    session
        .query(
            "UPDATE ecommerce.orders SET status = ? WHERE id = ?",
            ("completed", order.id.value())
        )
        .await?;

    println!("✓ Updated order status to 'completed'");

    // Query updated order
    let updated_order_rows = session
        .query(
            "SELECT id, user_id, product_id, quantity, total_price, status, order_date, notes FROM ecommerce.orders WHERE id = ?",
            (order.id.value(),)
        )
        .await?
        .rows
        .ok_or("No rows returned")?;

    for row in updated_order_rows.into_typed::<Order>() {
        let updated_order = row?;
        println!("Updated order status: {}", updated_order.status);
    }

    // Demonstrate type safety with Tagged types
    println!("\n=== Type Safety Demonstration ===");
    
    // This would cause a compile error if uncommented:
    // let invalid_assignment: UserId = product.id; // Error: cannot assign ProductId to UserId
    
    // But this works fine:
    let user_id_from_uuid: UserId = Uuid::new_v4().into();
    let product_id_from_uuid: ProductId = Uuid::new_v4().into();
    
    println!("✓ Type safety preserved: UserId and ProductId are distinct types");
    println!("  UserId: {}", user_id_from_uuid.value());
    println!("  ProductId: {}", product_id_from_uuid.value());

    println!("\n=== Example completed successfully! ===");
    println!("This example demonstrates:");
    println!("1. Tagged types for type-safe IDs and domain values");
    println!("2. FromRow derive working with Tagged types");
    println!("3. Mixed field types in CQL queries (Tagged + primitive types)");
    println!("4. Complex queries and relationships");
    println!("5. Type safety preventing ID mixups");

    Ok(())
}
