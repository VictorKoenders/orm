pub trait Column: Sized {
    type Type;
    fn name() -> &'static str;
}
