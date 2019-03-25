fn get_query_parameters<'a, 'b>(
    builder: &'b db_core::QueryBuilder<'a>,
) -> Vec<&'b Box<db_core::Argument<'a>>> {
    let mut result = Vec::with_capacity(builder.criteria.len());
    for criteria in &builder.criteria {
        if let db_core::FieldOrArgument::Argument(a) = &criteria.right {
            result.push(a);
        }
    }
    result
}

