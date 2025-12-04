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

/// Trait to enforce the use of Tagged types in function signatures.
/// This prevents accidental use of raw primitives and ensures type safety.
///
/// # Example
///
/// ```rust
/// use tagged_core::{Tagged, Taggable};
///
/// #[derive(Debug)]
/// struct UserIdTag;
/// type UserId = Tagged<u32, UserIdTag>;
///
/// fn process_user_id<T: Taggable>(id: T) {
///     // This function only accepts Tagged types, not raw u32
///     println!("Processing user ID: {:?}", id);
/// }
///
/// fn main() {
///     let user_id: UserId = 42.into();
///     process_user_id(user_id); // ✓ Works
///     // process_user_id(42);    // ✗ Compile error: expected Taggable, found integer
/// }
/// ```
pub trait Taggable {
    type Inner;
    type Tag;

    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self::Inner>()
    }
}

impl<T, Tag> Taggable for Tagged<T, Tag> {
    type Inner = T;
    type Tag = Tag;
}

impl<T: Default, Tag> Default for Tagged<T, Tag> {
    fn default() -> Self {
        Self { value: Default::default(), _marker: Default::default() }
    }
}


impl<T, Tag> Tagged<T, Tag> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            _marker: std::marker::PhantomData,
        }
    }

    /// ⚠️ **WARNING**: Avoid using `.value()` as it breaks type safety!
    /// 
    /// Using `.value()` extracts the inner value, which defeats the purpose of `Tagged` types.
    /// Instead, prefer:
    /// - Using `Deref` coercion: `let raw: &T = &*tagged;`
    /// - Using `Into` conversion: `let raw: T = tagged.into();`
    /// - Keeping values as `Tagged` types throughout your codebase
    /// 
    /// # Example (Avoid)
    /// ```rust,no_run
    /// # use tagged_core::Tagged;
    /// # struct UserIdTag;
    /// # type UserId = Tagged<u32, UserIdTag>;
    /// let user_id: UserId = 42.into();
    /// let raw = user_id.value(); // ⚠️ Breaks type safety!
    /// ```
    /// 
    /// # Example (Preferred)
    /// ```rust,no_run
    /// # use tagged_core::Tagged;
    /// # struct UserIdTag;
    /// # type UserId = Tagged<u32, UserIdTag>;
    /// let user_id: UserId = 42.into();
    /// let raw: u32 = user_id.into(); // ✓ Maintains type safety through conversion
    /// ```
    #[deprecated(
        since = "0.8.0",
        note = "Using .value() breaks type safety. Prefer Deref coercion (&*tagged) or Into conversion (tagged.into()) instead."
    )]
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

/// Support JSON string deserialization into `Tagged<T, Tag>`
/// 
/// # Example
/// 
/// ```rust,no_run
/// use tagged_core::Tagged;
/// use serde::Deserialize;
/// use std::convert::TryFrom;
/// 
/// #[derive(Debug, Deserialize)]
/// struct UserIdTag;
/// 
/// type UserId = Tagged<u32, UserIdTag>;
/// 
/// fn main() {
///     let json = "42";
///     let user_id: UserId = Tagged::from_json(json).unwrap();
///     println!("User ID: {}", user_id.value());
/// }
/// ```
#[cfg(feature = "serde")]
impl<T, Tag> Tagged<T, Tag>
where
    T: serde::de::DeserializeOwned,
{
    /// Deserialize a JSON string into a `Tagged` type
    /// 
    /// Requires the `serde` feature to be enabled.
    /// 
    /// # Errors
    /// 
    /// Returns a `serde_json::Error` if the JSON string cannot be deserialized into type `T`
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use tagged_core::Tagged;
    /// use serde::Deserialize;
    /// 
    /// #[derive(Debug, Deserialize)]
    /// struct UserIdTag;
    /// 
    /// type UserId = Tagged<u32, UserIdTag>;
    /// 
    /// fn main() {
    ///     let json = "42";
    ///     let user_id: UserId = Tagged::from_json(json).unwrap();
    ///     println!("User ID: {}", user_id.value());
    /// }
    /// ```
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json).map(Self::new)
    }

    /// Deserialize a JSON string from a `String` into a `Tagged` type
    /// 
    /// Requires the `serde` feature to be enabled.
    /// 
    /// # Errors
    /// 
    /// Returns a `serde_json::Error` if the JSON string cannot be deserialized into type `T`
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use tagged_core::Tagged;
    /// use serde::Deserialize;
    /// 
    /// #[derive(Debug, Deserialize)]
    /// struct UserIdTag;
    /// 
    /// type UserId = Tagged<u32, UserIdTag>;
    /// 
    /// fn main() {
    ///     let json = String::from("42");
    ///     let user_id: UserId = Tagged::from_json_string(json).unwrap();
    ///     println!("User ID: {}", user_id.value());
    /// }
    /// ```
    pub fn from_json_string(json: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&json).map(Self::new)
    }
}

#[cfg(feature = "serde")]
impl<T, Tag> Tagged<T, Tag>
where
    T: serde::Serialize,
{
    /// Serialize a `Tagged` type into a JSON string
    /// 
    /// Requires the `serde` feature to be enabled.
    /// 
    /// # Errors
    /// 
    /// Returns a `serde_json::Error` if the value cannot be serialized to JSON
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use tagged_core::Tagged;
    /// use serde::Serialize;
    /// 
    /// #[derive(Debug, Serialize)]
    /// struct UserIdTag;
    /// 
    /// type UserId = Tagged<u32, UserIdTag>;
    /// 
    /// fn main() {
    ///     let user_id: UserId = Tagged::from(42);
    ///     let json = user_id.to_json().unwrap();
    ///     println!("JSON: {}", json);
    /// }
    /// ```
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&**self)
    }

    /// Serialize a `Tagged` type into a pretty-printed JSON string
    /// 
    /// Requires the `serde` feature to be enabled.
    /// 
    /// # Errors
    /// 
    /// Returns a `serde_json::Error` if the value cannot be serialized to JSON
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use tagged_core::Tagged;
    /// use serde::Serialize;
    /// 
    /// #[derive(Debug, Serialize)]
    /// struct UserIdTag;
    /// 
    /// type UserId = Tagged<u32, UserIdTag>;
    /// 
    /// fn main() {
    ///     let user_id: UserId = Tagged::from(42);
    ///     let json = user_id.to_json_pretty().unwrap();
    ///     println!("JSON: {}", json);
    /// }
    /// ```
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&**self)
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
        (&**self).serialize(serializer)
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

/// This is just a marker type for macro transformation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id<T>(pub T);

// impl<T, U> scylla::_macro_internal::FromCqlVal<T> for Tagged<T, U>
// {
//     fn from_cql(cql_val: T) -> Result<Self, FromCqlValError> {
//         Self::new(cql_val.)
//     }
// }

// use scylla::frame::response::result::CqlValue;
// use scylla::impl_from_cql_value_from_method;
// // struct MyBytes(Vec<u8>);
//
// trait CqlValueExt {
//     fn into_my_bytes(self) -> Option<Tagged<Vec<u8>, Id<Self>>;
// }
//
// impl CqlValueExt for CqlValue {
//     fn into_my_bytes(self) -> Option<Tagged<Vec<u8>, Id<Self>> {
//         Some(MyBytes(self.into_blob()?))
//     }
// }
//
// impl_from_cql_value_from_method!(MyBytes, into_my_bytes);
// #[cfg(feature = "scylla")]
// impl<T: scylla::_macro_internal::FromCqlVal<scylla::_macro_internal::CqlValue>, U> scylla::_macro_internal::FromCqlVal<Tagged<scylla::_macro_internal::CqlValue, U>> for Tagged<T, U> {
//     fn from_cql(cql_val_opt: Tagged<scylla::_macro_internal::CqlValue, U>) -> Result<Self, scylla::_macro_internal::FromCqlValError> {
//         Ok(Self::new(T::from_cql(cql_val_opt.value)?))
//     }
// }

// impl<T, U> scylla::_macro_internal::FromRow for Tagged<T, U>
// where
//     T: scylla::_macro_internal::FromCqlVal<::std::option::Option<scylla::_macro_internal::CqlValue>>
// {
//     fn from_row(row: scylla::_macro_internal::Row) -> ::std::result::Result<Self, scylla::_macro_internal::FromRowError> {
//         use scylla::_macro_internal::{CqlValue, FromCqlVal, FromRow, FromRowError};
//         use ::std::result::Result::{Ok, Err};
//         use ::std::iter::{Iterator, IntoIterator};
//         if 4usize != row.columns.len() { return Err(FromRowError::WrongRowSize { expected: 4usize, actual: row.columns.len() }); }
//         let mut vals_iter = row.columns.into_iter().enumerate();
//         Ok(Tagged::new(
//             {
//                 let (col_ix, col_value) = vals_iter.next().unwrap();
//                 <T as FromCqlVal<::std::option::Option<CqlValue>>>::from_cql(col_value).map_err(|e| FromRowError::BadCqlVal { err: e, column: col_ix })?
//             },
//         ))
//     }
// }
//

// #[cfg(feature = "scylla")]
// impl<T: scylla::cql_to_rust::FromCqlVal<scylla::frame::response::result::CqlValue>, U> scylla::cql_to_rust::FromCqlVal<scylla::frame::response::result::CqlValue> for Tagged<T, U>
// {
//     fn from_cql(cql_val: scylla::frame::response::result::CqlValue) -> Result<Self, scylla::cql_to_rust::FromCqlValError> {
//         T::from_cql(cql_val).map(Self::new)
//     }
// }


// #[cfg(feature = "scylla")]
// impl<T: scylla::cql_to_rust::FromCqlVal<Option<scylla::frame::response::result::CqlValue>>, U> scylla::cql_to_rust::FromCqlVal<Option<scylla::frame::response::result::CqlValue>> for Tagged<T, U>
// {
//     fn from_cql(cql_val: Option<scylla::frame::response::result::CqlValue>) -> Result<Self, scylla::cql_to_rust::FromCqlValError> {
//         T::from_cql(cql_val).map(Self::new)
//     }
// }


#[cfg(feature = "scylla")]
impl<T: scylla::serialize::value::SerializeCql, U> scylla::serialize::value::SerializeCql for Tagged<T, U>
{
    fn serialize<'b>(
        &self,
        typ: &scylla::frame::response::result::ColumnType,
        writer: scylla::serialize::writers::CellWriter<'b>,
    ) -> Result<scylla::serialize::writers::WrittenCellProof<'b>, scylla::serialize::SerializationError> {
        self.value.serialize(typ, writer)
    }
}

#[cfg(feature = "scylla")]
impl<T: scylla::cql_to_rust::FromCqlVal<scylla::frame::response::result::CqlValue>, U> scylla::cql_to_rust::FromCqlVal<scylla::frame::response::result::CqlValue> for Tagged<T, U>
{
    fn from_cql(cql_val: scylla::frame::response::result::CqlValue) -> Result<Self, scylla::cql_to_rust::FromCqlValError> {
        T::from_cql(cql_val).map(Self::new)
    }
}




// impl SerializeCql for i16 {
//     impl_serialize_via_writer!(|me, typ, writer| {
//         exact_type_check!(typ, SmallInt);
//         writer.set_value(me.to_be_bytes().as_slice()).unwrap()
//     });
// }

// #[cfg(feature = "scylla")]
// impl<i16, U> SerializeCql for Tagged<i16, U> {
//     impl_serialize_via_writer!(|me, typ, writer| {
//         exact_type_check!(typ, SmallInt);
//         writer.set_value(me.value.to_be_bytes().as_slice()).unwrap()
//     });
// }

// #[cfg(feature = "scylla")]
// impl<T, U> scylla::_macro_internal::SerializeCql for Tagged<T, U>
// where
//     T: scylla::_macro_internal::SerializeCql,
// {
//     fn serialize<'b>(
//         &self,
//         type_: &scylla::_macro_internal::ColumnType,
//         writer: scylla::_macro_internal::CellWriter<'b>,
//     ) -> Result<scylla::_macro_internal::WrittenCellProof<'b>, scylla::serialize::SerializationError> {
//         self.value().serialize(type_, writer)
//     }
// }
//
// impl<T, U> FromCqlVal<Option<scylla::_macro_internal::CqlValue>> for Tagged<T, U>
// where
//     T: FromCqlVal<Option<scylla::_macro_internal::CqlValue>>,
// {
//     fn from_cql(val: Option<CqlValue>) -> Result<Self, FromCqlValError> {
//         T::from_cql(val).map(Tagged::new)
//     }
// }

// impl<T, U> scylla::cql_to_rust::FromCqlVal<T> for Tagged<T, U>
// where
//     T: scylla::cql_to_rust::FromCqlVal<T>,
// {
//     fn from_cql(cql_val: T) -> Result<Self, scylla::cql_to_rust::FromCqlValError> {
//         T::from_cql(cql_val).map(Self::new)
//     }
// }

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

