use serde::{Deserialize, Serialize};
use tagged_core::Tagged;

#[derive(Clone, Hash, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct SomeCustomType {
    some_id: String
}
#[derive(Clone, Hash, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct SomeCustomType2(String);
#[derive(Clone, Hash, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct User {
    id: Tagged<String, Self>,
    id2: SomeCustomType,
    id3: SomeCustomType2,
}


fn main() {
    let user = User { id: "1".into() , id2: SomeCustomType { some_id: "2".into() }, id3: SomeCustomType2("3".into())};
    let j = serde_json::to_string(&user).unwrap();
    let converted_user = serde_json::from_str::<User>(&j).unwrap();
    println!("{}", j);
    println!("{:?}", converted_user);
}
/*
 Running `target/debug/examples/Serde_example`
{"id":"1","id2":{"some_id":"2"},"id3":"3"}
User { id: "1", id2: SomeCustomType { some_id: "2" }, id3: SomeCustomType2("3") }

Process finished with exit code 0
*/

/*
Problem with normal types
{"id":"1","id2":{"some_id":"2"}}

// rust is powerful enough to solve it using touple 
{"id":"1","id2":{"some_id":"2"},"id3":"3"}

// or we can use a new type called tagged that don't need a new name.
*/