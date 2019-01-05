use crate::Result;

pub trait Queryable<T> {
    fn get_results(&self) -> Result<Vec<T>>;
    fn get_result(&self) -> Result<Option<T>> {
        Ok(self.get_results()?.into_iter().next())
    }
}
