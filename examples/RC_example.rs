use rust_tagged::Tagged;
use std::rc::Rc;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Org;

type OrgRef = Tagged<Rc<String>, Org>;

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    name: String,
    org: OrgRef,
}

fn main() {
    let shared = Rc::new("codefonsi.com".to_string());

    let project = Project {
        name: "Open Source".into(),
        org: shared.clone().into(),
    };

    let json = serde_json::to_string(&project).unwrap();
    println!("Serialized: {json}");
}