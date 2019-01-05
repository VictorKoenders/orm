pub trait Connection: postgres::GenericConnection {}

impl<'a> Connection for postgres::transaction::Transaction<'a> {}
impl<'a> Connection for postgres::Connection {}
