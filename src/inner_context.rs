use crate::Result;
use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

#[derive(Clone)]
pub struct InnerContext {
    pub pool: Pool<PostgresConnectionManager>,
}

impl InnerContext {
    pub fn new(url: &str) -> Result<Self> {
        let manager = PostgresConnectionManager::new(url, TlsMode::None)?;
        let pool = Pool::new(manager)?;
        Ok(InnerContext { pool })
    }
}
