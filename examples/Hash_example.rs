
fn main() {
    use tagged_core::Tagged;
    use std::collections::HashSet;

    #[derive(Clone, Hash, Debug, PartialEq, Eq)]
    struct User {
        id: Tagged<String, Self>
    }
    let mut s: HashSet<User> = HashSet::new();
    let user = User{id: "me@example.com".into()};
    s.insert(user.clone());

    assert!(s.contains(&user));
}