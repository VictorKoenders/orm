pub trait ExpressionNext<COLUMN, T> {
    type Result;

    fn next(self, val: T) -> Self::Result;
}
