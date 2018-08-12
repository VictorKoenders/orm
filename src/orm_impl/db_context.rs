use super::Result;

pub trait DbContext : Sized {
    fn connect(url: impl AsRef<str>) -> Result<Self>;
}
