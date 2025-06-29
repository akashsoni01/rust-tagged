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
