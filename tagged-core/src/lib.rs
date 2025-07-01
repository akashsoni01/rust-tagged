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
/// # Example
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

impl<T: Hash, Tag> Hash for Tagged<T, Tag> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}


#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize, Serializer, Deserializer};

#[cfg(feature = "serde")]
impl<T: Serialize, Tag> Serialize for Tagged<T, Tag> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.value().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T: Deserialize<'de>, Tag> Deserialize<'de> for Tagged<T, Tag> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        T::deserialize(deserializer).map(Self::new)
    }
}

impl<T, Tag> IntoIterator for Tagged<Vec<T>, Tag> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

impl<'a, T, Tag> IntoIterator for &'a Tagged<Vec<T>, Tag> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.iter()
    }
}


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

