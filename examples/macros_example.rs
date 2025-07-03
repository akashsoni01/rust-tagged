use tagged_core::{Id, Tagged};
use tagged_macros::WithId;

#[derive(WithId)]
struct Foo {
    id: Id<u32>,
}

fn main() {

}