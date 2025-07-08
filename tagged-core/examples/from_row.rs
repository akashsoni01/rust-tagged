use scylla::FromRow;
use tagged_core::{Tagged, self};

#[derive(Debug, FromRow)]
struct UserRow {
    id: Tagged<i32, Self>,
    name: Option<Tagged<String, Self>>,
}

fn main() {
    // ..... your code here
}