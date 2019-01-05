use crate::InnerContext;

pub trait QueryBuilder {
    fn new(db: InnerContext) -> Self;
}
