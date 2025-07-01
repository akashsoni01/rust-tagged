use tagged_core::Tagged;


#[derive(Debug)]
struct UserIdTag {
    a: Tagged<u32, Self>,
    b: Tagged<u32, Self>,
}


fn main() {
    let instance = UserIdTag{a: 1.into(), b: 2.into()};

    println!("{}", instance.a);
    println!("{:?}", instance.b);
}
