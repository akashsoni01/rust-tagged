use serde::{Deserialize, Serialize};

fn main() {
    use tagged_core::Tagged;
    #[derive(Clone, Hash, Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct SomeCustomType {
        some_id: String
    }
    #[derive(Clone, Hash, Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct SomeCustomType2(String);
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct User {
        id: Tagged<f32, Self>,
        id2: SomeCustomType,
        id3: SomeCustomType2,
    }

    let user = User { id: 0.0.into() , id2: SomeCustomType { some_id: "2".into() }, id3: SomeCustomType2("3".into())};
    let j = serde_json::to_string(&user).unwrap();
    println!("{}", j);
}

/*
Problem with normal types
{"id":"1","id2":{"some_id":"2"}}

// rust is powerful enough to solve it using touple
{"id":"1","id2":{"some_id":"2"},"id3":"3"}

// or we can use a new type called tagged that don't need a new name.
*/