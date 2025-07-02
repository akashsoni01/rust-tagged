use tagged_core::Tagged;

#[derive(Debug)]
struct Org;

type EmployeeNames = Tagged<Vec<String>, Org>;

fn main() {
    let names: EmployeeNames = Tagged::new(vec!["Alice".into(), "Bob".into()]);

    for name in &names {
        println!("Name: {name}");
    }

    // Consuming iterator
    for name in names {
        println!("Owned: {name}");
    }
}

/*
Name: Alice
Name: Bob
Owned: Alice
Owned: Bob
*/