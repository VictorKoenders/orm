use crate::Result;

pub trait Row {
    fn read_at_index<T: ReadType>(&mut self, index: usize) -> Result<T>;
    fn read_by_name<T: ReadType>(&mut self, name: &str) -> Result<T>;
}

pub trait ReadType: Sized {
    fn from_pq_bytes(b: Option<&[u8]>) -> Result<Self>;
}
