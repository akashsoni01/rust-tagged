use std::ops::Deref;
pub struct Tagged<T, U> {
    value: T,
    _marker: std::marker::PhantomData<U>,
}

impl<T, U> Tagged<T, U> {
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


/// Blanket `From<T>` for `Tagged<T, U>`
impl<T, U> From<T> for Tagged<T, U> {
    fn from(value: T) -> Self {
        Tagged::new(value)
    }
}

/// Support `From<&str>` → `Tagged<String, U>`
impl<U> From<&str> for Tagged<String, U> {
    fn from(s: &str) -> Self {
        Tagged::new(s.to_string())
    }
}

/// Support `From<&String>` → `Tagged<String, U>`
impl<U> From<&String> for Tagged<String, U> {
    fn from(s: &String) -> Self {
        Tagged::new(s.clone())
    }
}

impl<T, U> Deref for Tagged<T, U> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

// For all common primitive types
// macro_rules! impl_from_tagged {
//     ($($t:ty),*) => {
//         $(
//             impl<U> From<Tagged<$t, U>> for $t {
//                 fn from(tagged: Tagged<$t, U>) -> Self {
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