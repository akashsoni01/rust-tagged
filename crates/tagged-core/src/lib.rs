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