#[test]
pub fn test_pg_query() {
    use db_core::connection::QueryResult;
    use db_core::table_builder::{Column, ColumnFlags, ColumnType, Table};
    use db_postgres::ConnectionTrait;

    let conn =
        db_postgres::Connection::connect("postgres://trangar:Development@localhost/test").unwrap();

    let mut database_updater = conn.get_existing_schema().unwrap();
    database_updater.ensure_table_exists(Table {
        name: "user".into(),
        constraints: vec![],
        columns: vec![
            Column {
                name: "id".into(),
                default: None,
                flags: ColumnFlags::PRIMARY | ColumnFlags::INDEX | ColumnFlags::NOT_NULL,
                foreign_keys: vec![],
                r#type: ColumnType::BigInt,
            },
            Column {
                name: "name".into(),
                default: None,
                flags: ColumnFlags::NOT_NULL,
                foreign_keys: vec![],
                r#type: ColumnType::Text(Some(50)),
            },
            Column {
                name: "date_of_birth".into(),
                default: None,
                flags: ColumnFlags::empty(),
                foreign_keys: vec![],
                r#type: ColumnType::Custom("TIMESTAMPTZ".into()),
            },
        ],
    });
    let diff = database_updater.diff();

    conn.update_schema(&diff).expect("Could not update schema");

    let query_builder = db_core::query_builder::QueryBuilder {
        table: "user".into(),
        joined_tables: Vec::new(),
        select: vec![
            db_core::query_builder::Field {
                field: "id".into(),
                table: None,
                alias: None,
            },
            db_core::query_builder::Field {
                field: "name".into(),
                table: None,
                alias: None,
            },
            db_core::query_builder::Field {
                field: "date_of_birth".into(),
                table: None,
                alias: None,
            },
        ],
        criteria: vec![db_core::query_builder::Criteria {
            left: db_core::query_builder::Field {
                field: "id".into(),
                table: None,
                alias: None,
            },
            right: db_core::query_builder::FieldOrArgument::Argument(Box::new(String::from(
                "a4e52274-cf87-46f3-87fd-f22234064d1c",
            ))),
            comparison: db_core::query_builder::Comparison::EqualTo,
        }],
    };
    let mut result = conn.execute(query_builder).expect("OK");
    println!("{}", result.error_message());
    println!("Result len: {:?}", result.len());
}
