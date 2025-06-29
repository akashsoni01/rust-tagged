use tagged_core::{Tagged};

struct UserIdTag;
fn main() {
    let id = Tagged::<u32, UserIdTag>::new(42);
    println!("Tagged value: {}", id.value());
}
