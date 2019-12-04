use std::marker::PhantomData;

pub struct Lazy<T> {
    pd: PhantomData<T>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for Lazy<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "Lazy<T>")
    }
}
