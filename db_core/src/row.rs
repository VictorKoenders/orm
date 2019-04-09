use crate::Result;

pub trait Row {
    fn read_at_index<'a, T: ReadType<'a>>(&'a self, index: usize) -> Result<T>;
    fn read_by_name<'a, T: ReadType<'a>>(&'a self, name: &str) -> Result<T>;
}

// TODO: Could this be merged with db_core::query_builder::Argument?
pub trait ReadType<'a>: Sized {
    fn from_pq_bytes(b: Option<&'a [u8]>) -> Result<Self>;
}

impl<'a> ReadType<'a> for bool {
    fn from_pq_bytes(b: Option<&'a [u8]>) -> Result<Self> {
        if let Some(last_byte) = b.and_then(|b| b.last()) {
            return Ok(*last_byte > 0)
        }
        failure::bail!("Could not load NULL value into non-nullable bool")
    }
}

impl<'a> ReadType<'a> for Option<bool> {
    fn from_pq_bytes(b: Option<&'a [u8]>) -> Result<Self> {
        Ok(b.and_then(|b| b.last()).map(|last_byte| *last_byte > 0))
    }
}

impl<'a> ReadType<'a> for &'a str {
    fn from_pq_bytes(b: Option<&'a [u8]>) -> Result<Self> {
        match b.map(std::str::from_utf8) {
            Some(Ok(s)) => Ok(s),
            Some(Err(e)) => failure::bail!("UTF8 str {:?} is invalid: {:?}", b.unwrap(), e),
            None => failure::bail!("Could not load NULL value into non-nullable string"),
        }
    }
}
impl<'a> ReadType<'a> for Option<&'a str> {
    fn from_pq_bytes(b: Option<&'a [u8]>) -> Result<Self> {
        match b.map(std::str::from_utf8) {
            Some(Ok(s)) => Ok(Some(s)),
            Some(Err(e)) => failure::bail!("UTF8 str {:?} is invalid: {:?}", b.unwrap(), e),
            None => Ok(None),
        }
    }
}

impl<'a> ReadType<'a> for String {
    fn from_pq_bytes(b: Option<&'a [u8]>) -> Result<Self> {
        match b.map(std::str::from_utf8) {
            Some(Ok(s)) => Ok(s.to_owned()),
            Some(Err(e)) => failure::bail!("UTF8 str {:?} is invalid: {:?}", b.unwrap(), e),
            None => failure::bail!("Could not load NULL value into non-nullable string"),
        }
    }
}
impl<'a> ReadType<'a> for Option<String> {
    fn from_pq_bytes(b: Option<&'a [u8]>) -> Result<Self> {
        match b.map(std::str::from_utf8) {
            Some(Ok(s)) => Ok(Some(s.to_owned())),
            Some(Err(e)) => failure::bail!("UTF8 str {:?} is invalid: {:?}", b.unwrap(), e),
            None => Ok(None),
        }
    }
}
