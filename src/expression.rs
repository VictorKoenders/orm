use crate::{Column, DbEquals, Eq, ExpressionNext};
use std::marker::PhantomData;

pub struct Expression<PARENT, COLUMN> {
    parent: PARENT,
    _column: PhantomData<COLUMN>,
}

impl<PARENT, COLUMN> Expression<PARENT, COLUMN> {
    pub fn new(parent: PARENT) -> Self {
        Self {
            parent,
            _column: PhantomData,
        }
    }

    pub fn eq<VAL>(self, val: VAL) -> <PARENT as ExpressionNext<COLUMN, Eq<VAL>>>::Result
    where
        PARENT: ExpressionNext<COLUMN, Eq<VAL>>,
        COLUMN: Column,
        VAL: DbEquals<<COLUMN as Column>::Type>,
    {
        self.parent.next(Eq { val })
    }
}
