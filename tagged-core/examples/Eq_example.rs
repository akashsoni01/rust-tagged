use tagged_core::Tagged;
fn main() {}
#[test]
fn test_eq() {
    let a = Tagged::<u32, String>::new(42);
    let b = Tagged::<u32, String>::new(42);
    let c = Tagged::<u32, String>::new(35);
    let d = Tagged::<u32, String>::new(0);

    assert!(a == b);
    assert!(a > c);
    assert!(a >= d);

}

#[test]
fn test_with_struct() {
    use tagged_core::Tagged;

    struct UserIdTag {
        a: Tagged<u32, Self>,
        b: Tagged<u32, Self>,
    }


    let instance = UserIdTag{a: 1.into(), b: 2.into()};

    assert!(instance.a < instance.b);

    // println!("{}", a);

}