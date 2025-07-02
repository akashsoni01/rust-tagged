/// rust-tagged provides a simple way to define strongly typed wrappers over primitive types like String, i32, Uuid, chrono::DateTime, etc. It helps eliminate bugs caused by misusing raw primitives for conceptually distinct fields such as UserId, Email, ProductId, and more.
/// 
/// Eliminate accidental mixups between similar types (e.g. OrgId vs UserId)
/// Enforce domain modeling in code via the type system
/// Ergonomic .into() support for primitive conversions
/// 
/// # Example - Simple 
/// 
/// ```
/// use tagged_core::{Tagged};
/// 
/// #[derive(Debug)]
/// struct EmailTag;
/// 
/// type Email = Tagged<String, EmailTag>;
/// 
/// fn main() {
///     let email: Email = "test@example.com".into();
///     println!("Email inner value: {}", email.value());
/// 
///     // Convert back to String
///     let raw: String = email.into();
///     println!("Raw String: {raw}");
/// }
/// ```
/// 
/// # Example - Debug
/// ```
/// use tagged_core::Tagged;
/// 
/// 
/// #[derive(Debug)]
/// struct UserIdTag {
///     a: Tagged<u32, Self>,
///     b: Tagged<u32, Self>,
/// }
/// 
/// 
/// fn main() {
///     let instance = UserIdTag{a: 1.into(), b: 2.into()};
/// 
///     println!("{}", instance.a);
///     println!("{:?}", instance.b);
/// }
/// ```
/// 
/// # Example - Hash
/// ```
/// fn main() {
///     use tagged_core::Tagged;
///     use std::collections::HashSet;
/// 
///     #[derive(Clone, Hash, Debug, PartialEq, Eq)]
///     struct User {
///         id: Tagged<String, Self>
///     }
///     let mut s: HashSet<User> = HashSet::new();
///     let user = User{id: "me@example.com".into()};
///     s.insert(user.clone());
/// 
///     assert!(s.contains(&user));
/// }
/// ```
/// 
/// # Example - Iter
/// ```
/// use tagged_core::Tagged;
/// 
/// #[derive(Debug)]
/// struct Org;
/// 
/// type EmployeeNames = Tagged<Vec<String>, Org>;
/// 
/// fn main() {
///     let names: EmployeeNames = Tagged::new(vec!["Alice".into(), "Bob".into()]);
/// 
///     for name in &names {
///         println!("Name: {name}");
///     }
/// 
///     // Consuming iterator
///     for name in names {
///         println!("Owned: {name}");
///     }
/// }
/// 
/// /*
/// Name: Alice
/// Name: Bob
/// Owned: Alice
/// Owned: Bob
/// */
/// ```
/// 
/// # Example - Mutation
/// ```
/// use tagged_core::Tagged;
/// 
/// #[derive(Debug)]
/// struct Org;
/// 
/// type OrgName = Tagged<String, Org>;
/// 
/// fn main() {
///     let mut name = OrgName::new("Codefonsi".into());
/// 
///     name.set("New Org Name".into());
/// 
///     println!("Updated Org Name: {}", name.value());
/// }
/// ```

pub use tagged_core::*;
