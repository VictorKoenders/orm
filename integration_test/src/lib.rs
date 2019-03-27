#[test]
pub fn test_pg_query() {
    use db_postgres::ConnectionTrait;
    use db_core::QueryResult;

    let conn = db_postgres::Connection::connect("postgres://trangar:Development@localhost/test").unwrap();
    let query_builder = db_core::QueryBuilder {
        table: "user".into(),
        joined_tables: Vec::new(),
        select: vec![
            db_core::Field {
                field: "id".into(),
                table: None,
                alias: None,
            },
            db_core::Field {
                field: "name".into(),
                table: None,
                alias: None,
            },
            db_core::Field {
                field: "date_of_birth".into(),
                table: None,
                alias: None,
            },
        ],
        criteria: vec![
            db_core::Criteria {
                left: db_core::Field {
                    field: "id".into(),
                    table: None,
                    alias: None,
                },
                right: db_core::FieldOrArgument::Argument(Box::new(5i32)),
                comparison: db_core::Comparison::EqualTo,
            }
        ]
    };
    let mut result = conn.execute(query_builder).expect("OK");
    println!("{}", result.error_message());
    println!("Result len: {:?}", result.len());
}
