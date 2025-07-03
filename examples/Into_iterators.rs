use tagged_core::Tagged;

#[derive(Debug)]
struct Org;

type EmployeeNames = Tagged<Vec<String>, Org>;

fn main() {
    let names: EmployeeNames = Tagged::new(vec!["Alice".into(), "Bob".into()]);
    names.into_iter().for_each(|name| println!("Name: {}", name));
}

/*
Name: Alice
Name: Bob
*/