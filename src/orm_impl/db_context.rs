use super::Result;

/// A database context. This is automatically implemented if you annotate your struct with `#[derive(DbContext)]`.
/// 
/// This context defines a single method [connect](#tymethod.connect). This method takes anything that looks like a connection string, and returns your Context.
pub trait DbContext: Sized {
    /// Take anything that looks like a connection string, and returns a DbContext.
    fn connect(url: impl AsRef<str>) -> Result<Self>;
}
