use scylla::{Session, SessionBuilder};
use scylla::transport::errors::QueryError;
use tagged_core::Tagged;

#[derive(Debug)]
struct UserRow;
type UserId = Tagged<i32, UserRow>;

async fn test() -> Result<(), QueryError> {
    let session: Session = SessionBuilder::new()
        .known_node("127.0.0.1:9042")
        .build()
        .await.unwrap();

    let id: UserId = 42.into();

    // This line only works if `Tagged<i32, _>` implements `scylla::Value`
    let result = session
        .query("SELECT * FROM users WHERE id = ?", (id,))
        .await?;

    Ok(())
}

fn main() {
    
}
