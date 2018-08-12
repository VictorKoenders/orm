use super::{Table, Result};

pub trait Queryable<TABLE: Table> {
    fn generate_query(&self, out: &mut String) -> Result<()>;

    fn execute(&self) -> Result<Vec<TABLE>> {
        let mut str = String::new();
        self.generate_query(&mut str)?;
        println!("{:?}", str);
        Ok(Vec::new())
    }
}
