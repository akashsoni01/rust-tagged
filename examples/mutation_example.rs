use tagged_core::Tagged;

#[derive(Debug)]
struct Org;

type OrgName = Tagged<String, Org>;

fn main() {
    let mut name = OrgName::new("Codefonsi".into());

    name.set("New Org Name".into());

    println!("Updated Org Name: {}", &*name);
}
