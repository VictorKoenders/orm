use crate::Result;

pub trait Row {
    fn read_at_index<T: ReadType>(&mut self, index: usize) -> Result<T>;
    fn read_by_name<T: ReadType>(&mut self, name: &str) -> Result<T>;
}

// TODO: Could this be merged with db_core::query_builder::Argument?
pub trait ReadType: Sized {
    fn from_pq_bytes(b: Option<&[u8]>) -> Result<Self>;
}

impl ReadType for bool {
    fn from_pq_bytes(b: Option<&[u8]>) -> Result<Self> {
        match b.map(std::str::from_utf8) {
            Some(Ok(s)) => Ok(s == "YES"),
            Some(Err(e)) => failure::bail!("UTF8 str {:?} is invalid: {:?}", b.unwrap(), e),
            None => failure::bail!("Could not load NULL value into non-nullable bool"),
        }
    }
}

impl ReadType for Option<bool> {
    fn from_pq_bytes(b: Option<&[u8]>) -> Result<Self> {
        match b.map(std::str::from_utf8) {
            Some(Ok(s)) => Ok(Some(s == "YES")),
            Some(Err(e)) => failure::bail!("UTF8 str {:?} is invalid: {:?}", b.unwrap(), e),
            None => Ok(None),
        }
    }
}

impl ReadType for String {
    fn from_pq_bytes(b: Option<&[u8]>) -> Result<Self> {
        match b.map(std::str::from_utf8) {
            Some(Ok(s)) => Ok(s.to_owned()),
            Some(Err(e)) => failure::bail!("UTF8 str {:?} is invalid: {:?}", b.unwrap(), e),
            None => failure::bail!("Could not load NULL value into non-nullable string"),
        }
    }
}
impl ReadType for Option<String> {
    fn from_pq_bytes(b: Option<&[u8]>) -> Result<Self> {
        match b.map(std::str::from_utf8) {
            Some(Ok(s)) => Ok(Some(s.to_owned())),
            Some(Err(e)) => failure::bail!("UTF8 str {:?} is invalid: {:?}", b.unwrap(), e),
            None => Ok(None),
        }
    }
}
