use std::cmp::Ordering;
use std::fmt;
use std::ops::Deref;
use std::hash::{Hash, Hasher};

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
pub struct Tagged<T, Tag> {
    value: T,
    _marker: std::marker::PhantomData<Tag>,
}

impl<T, Tag> Tagged<T, Tag> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}


/// Blanket `From<T>` for `Tagged<T, Tag>`
impl<T, Tag> From<T> for Tagged<T, Tag> {
    fn from(value: T) -> Self {
        Tagged::new(value)
    }
}

/// Support `From<&str>` → `Tagged<String, Tag>`
impl<Tag> From<&str> for Tagged<String, Tag> {
    fn from(s: &str) -> Self {
        Tagged::new(s.to_string())
    }
}

/// Support `From<&String>` → `Tagged<String, Tag>`
impl<Tag> From<&String> for Tagged<String, Tag> {
    fn from(s: &String) -> Self {
        Tagged::new(s.clone())
    }
}

impl<T, Tag> Deref for Tagged<T, Tag> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T: PartialEq, Tag> PartialEq for Tagged<T, Tag> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq, Tag> Eq for Tagged<T, Tag> {}

impl<T: PartialOrd, Tag> PartialOrd for Tagged<T, Tag> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T: Ord, Tag> Ord for Tagged<T, Tag> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

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
impl<T: fmt::Debug, Tag> fmt::Debug for Tagged<T, Tag> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<T: fmt::Display, Tag> fmt::Display for Tagged<T, Tag> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<T: Clone, Tag> Clone for Tagged<T, Tag> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            _marker: std::marker::PhantomData,
        }
    }
}

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
impl<T: Hash, Tag> Hash for Tagged<T, Tag> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}


#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize, Serializer, Deserializer};


/// Example - Serialize
/// ```
/// use serde::{Deserialize, Serialize};
/// use tagged_core::Tagged;
///
/// #[derive(Clone, Hash, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// struct SomeCustomType {
///     some_id: String
/// }
/// #[derive(Clone, Hash, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// struct SomeCustomType2(String);
/// #[derive(Clone, Hash, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// struct User {
///     id: Tagged<String, Self>,
///     id2: SomeCustomType,
///     id3: SomeCustomType2,
/// }
///
///
/// fn main() {
///     let user = User { id: "1".into() , id2: SomeCustomType { some_id: "2".into() }, id3: SomeCustomType2("3".into())};
///     let j = serde_json::to_string(&user).unwrap();
///     println!("{}", j);
/// }
///
/// /*
/// Problem with normal types
/// {"id":"1","id2":{"some_id":"2"}}
///
/// // rust is powerful enough to solve it using touple
/// {"id":"1","id2":{"some_id":"2"},"id3":"3"}
///
/// // or we can use a new type called tagged that don't need a new name.
/// */
/// ```
#[cfg(feature = "serde")]
impl<T: Serialize, Tag> Serialize for Tagged<T, Tag> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.value().serialize(serializer)
    }
}


/// ```
/// use serde::{Deserialize, Serialize};
/// use tagged_core::Tagged;
/// 
/// #[derive(Clone, Hash, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// struct SomeCustomType {
///     some_id: String
/// }
/// #[derive(Clone, Hash, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// struct SomeCustomType2(String);
/// #[derive(Clone, Hash, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// struct User {
///     id: Tagged<String, Self>,
///     id2: SomeCustomType,
///     id3: SomeCustomType2,
/// }
/// 
/// 
/// fn main() {
///     let user = User { id: "1".into() , id2: SomeCustomType { some_id: "2".into() }, id3: SomeCustomType2("3".into())};
///     let j = serde_json::to_string(&user).unwrap();
///     let converted_user = serde_json::from_str::<User>(&j).unwrap();
///     println!("{}", j);
///     println!("{:?}", converted_user);
/// }
/// /*
///  Running `target/debug/examples/Serde_example`
/// {"id":"1","id2":{"some_id":"2"},"id3":"3"}
/// User { id: "1", id2: SomeCustomType { some_id: "2" }, id3: SomeCustomType2("3") }
/// 
/// Process finished with exit code 0
/// */
/// 
/// /*
/// Problem with normal types
/// {"id":"1","id2":{"some_id":"2"}}
/// 
/// // rust is powerful enough to solve it using touple 
/// {"id":"1","id2":{"some_id":"2"},"id3":"3"}
/// 
/// // or we can use a new type called tagged that don't need a new name.
/// */
/// ```
#[cfg(feature = "serde")]
impl<'de, T: Deserialize<'de>, Tag> Deserialize<'de> for Tagged<T, Tag> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        T::deserialize(deserializer).map(Self::new)
    }
}

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
///     names.into_iter().for_each(|name| println!("Name: {}", name));
/// }
/// 
/// /*
/// Name: Alice
/// Name: Bob
/// */
/// ```
impl<T, Tag> IntoIterator for Tagged<Vec<T>, Tag> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

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
///     names.iter().for_each(|name| println!("Name: {}", name));
/// }
/// 
/// /*
/// Name: Alice
/// Name: Bob
/// */
/// ```
impl<'a, T, Tag> IntoIterator for &'a Tagged<Vec<T>, Tag> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.iter()
    }
}


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
impl<T, Tag> Tagged<T, Tag> {
    /// Not allowed feature - Get a mutable reference to the internal value
    // pub fn value_mut(&mut self) -> &mut T {
    //     &mut self.value
    // }

    /// Replace the inner value
    pub fn set(&mut self, new_value: T) {
        self.value = new_value;
    }
}


// For all common primitive types
// macro_rules! impl_from_tagged {
//     ($($t:ty),*) => {
//         $(
//             impl<Tag> From<Tagged<$t, Tag>> for $t {
//                 fn from(tagged: Tagged<$t, Tag>) -> Self {
//                     tagged.value
//                 }
//             }
//         )*
//     };
// }
// 
// impl_from_tagged!(
//     u8, u16, u32, u64, u128,
//     i8, i16, i32, i64, i128,
//     f32, f64,
//     usize, isize,
//     bool, char
// );


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        struct SomeStruct{
            id: Tagged<i32, SomeStruct>
        }
        let tagged_struct = SomeStruct{
            id: Tagged::new(0)
        };

        // tagged_strut.id = 3; //

        assert_eq!(tagged_struct.id.value, 0);
    }
}

