use crate::Result;

pub trait Row {
    fn read_string_at_index(&mut self, index: usize) -> Result<String>;
    fn read_i8_at_index(&mut self, index: usize) -> Result<i8>{
        self.read_string_at_index(index).and_then(|s| s.parse().map_err(Into::into))
    }
    fn read_i16_at_index(&mut self, index: usize) -> Result<i16>{
        self.read_string_at_index(index).and_then(|s| s.parse().map_err(Into::into))
    }
    fn read_i32_at_index(&mut self, index: usize) -> Result<i32>{
        self.read_string_at_index(index).and_then(|s| s.parse().map_err(Into::into))
    }
    fn read_i64_at_index(&mut self, index: usize) -> Result<i64>{
        self.read_string_at_index(index).and_then(|s| s.parse().map_err(Into::into))
    }

    fn read_string_by_name(&mut self, name: &str) -> Result<String>;
    fn read_i8_by_name(&mut self, name: &str) -> Result<i8>{
        self.read_string_by_name(name).and_then(|s| s.parse().map_err(Into::into))
    }
    fn read_i16_by_name(&mut self, name: &str) -> Result<i16>{
        self.read_string_by_name(name).and_then(|s| s.parse().map_err(Into::into))
    }
    fn read_i32_by_name(&mut self, name: &str) -> Result<i32>{
        self.read_string_by_name(name).and_then(|s| s.parse().map_err(Into::into))
    }
    fn read_i64_by_name(&mut self, name: &str) -> Result<i64>{
        self.read_string_by_name(name).and_then(|s| s.parse().map_err(Into::into))
    }

}

