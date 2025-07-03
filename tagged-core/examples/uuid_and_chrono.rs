use chrono::{DateTime, Utc};
use uuid::{Uuid};
use tagged_core::Tagged;

#[derive(Debug)]
struct SomeStruct {
    id: Tagged<Uuid, Self>,
    time_id: Tagged<DateTime<Utc>, Self>,
}

impl SomeStruct {
    fn new(id: Uuid, email: DateTime<Utc>) -> Self {
        Self {
            id: id.into(),
            time_id: email.into(),
        }
    }
}



fn main() {
    let id = Uuid::new_v4();
    let instance = SomeStruct::new(id , Utc::now());
    println!("{:?}", instance);
    
}

/*
SomeStruct { id: 15e74217-09a3-453f-b9f1-c47c3df84c34, time_id: 2025-07-02T03:08:13.726417Z }
*/