use tagged_core::{Tagged, Tag};

struct UserIdTag;
impl Tag for UserIdTag {}

fn main() {
    let id = Tagged::<u32, UserIdTag>::new(42);
    println!("Tagged value: {}", id.value());
}
