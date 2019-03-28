use crate::Result;
use std::borrow::Cow;

pub struct QueryBuilder<'a> {
    pub table: Cow<'a, str>,
    pub joined_tables: Vec<TableJoin<'a>>,
    pub select: Vec<Field<'a>>,
    pub criteria: Vec<Criteria<'a>>,
}

pub trait EstimateStrLen {
    fn estimate_str_len(&self) -> usize;
}

impl EstimateStrLen for QueryBuilder<'_> {
    fn estimate_str_len(&self) -> usize {
        self.table.len()
            + self
                .joined_tables
                .iter()
                .map(EstimateStrLen::estimate_str_len)
                .sum::<usize>()
            + self
                .select
                .iter()
                .map(EstimateStrLen::estimate_str_len)
                .sum::<usize>()
    }
}

pub struct Field<'a> {
    pub table: Option<Cow<'a, str>>,
    pub field: Cow<'a, str>,
    pub alias: Option<Cow<'a, str>>,
}

impl EstimateStrLen for Field<'_> {
    fn estimate_str_len(&self) -> usize {
        let mut result = 0;
        if let Some(table) = self.table.as_ref() {
            result += table.len();
        }
        result += self.field.len();
        if let Some(alias) = self.alias.as_ref() {
            result += alias.len();
        }
        result
    }
}

pub enum FieldOrArgument<'a> {
    Field(Field<'a>),
    Argument(Box<Argument<'a>>),
}

impl EstimateStrLen for FieldOrArgument<'_> {
    fn estimate_str_len(&self) -> usize {
        match self {
            FieldOrArgument::Field(f) => f.estimate_str_len(),
            FieldOrArgument::Argument(a) => a.estimate_str_len(),
        }
    }
}

pub trait Argument<'a> {
    fn estimate_str_len(&self) -> usize {
        0
    }
    fn to_query_string(&self) -> String;
    fn try_parse_from_query(str: &str) -> Result<Self>
    where
        Self: Sized;
}

impl<'a> Argument<'a> for i32 {
    fn to_query_string(&self) -> String {
        self.to_string()
    }
    fn try_parse_from_query(str: &str) -> Result<Self> {
        str.parse().map_err(Into::into)
    }
}

pub struct TableJoin<'a> {
    pub joined_table: Cow<'a, str>,
    pub parent_table: Cow<'a, str>,
    pub criteria: Vec<Criteria<'a>>,
}

impl EstimateStrLen for TableJoin<'_> {
    fn estimate_str_len(&self) -> usize {
        self.joined_table.len()
            + self.parent_table.len()
            + self
                .criteria
                .iter()
                .map(EstimateStrLen::estimate_str_len)
                .sum::<usize>()
    }
}

pub struct Criteria<'a> {
    pub left: Field<'a>,
    pub right: FieldOrArgument<'a>,
    pub comparison: Comparison<'a>,
}

impl EstimateStrLen for Criteria<'_> {
    fn estimate_str_len(&self) -> usize {
        self.left.estimate_str_len()
            + self.right.estimate_str_len()
            + self.comparison.estimate_str_len()
    }
}

pub enum Comparison<'a> {
    EqualTo,
    GreaterThen,
    GreaterOrEqualTo,
    LesserThen,
    LesserOrEqualTo,
    NotEqualTo,
    Like,
    Custom(Cow<'a, str>),
}

impl<'a> Comparison<'a> {
    pub fn as_query_str(&'a self) -> &'a str {
        match self {
            Comparison::EqualTo => "=",
            Comparison::GreaterThen => ">",
            Comparison::GreaterOrEqualTo => ">=",
            Comparison::LesserThen => "<",
            Comparison::LesserOrEqualTo => "<=",
            Comparison::NotEqualTo => "!=",
            Comparison::Like => " LIKE ",
            Comparison::Custom(s) => &s,
        }
    }
}

impl EstimateStrLen for Comparison<'_> {
    fn estimate_str_len(&self) -> usize {
        match self {
            Comparison::Custom(c) => c.len(),
            Comparison::Like => 4,
            _ => 2,
        }
    }
}
