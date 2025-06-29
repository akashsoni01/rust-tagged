pub trait Tag {}

pub struct Tagged<T, U: Tag> {
    value: T,
    _marker: std::marker::PhantomData<U>,
}

impl<T, U: Tag> Tagged<T, U> {
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
