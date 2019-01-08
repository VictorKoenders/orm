use crate::data::{Field, Table};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Result;

pub fn generate_implementation(table: &Table) -> Result<TokenStream> {
    let query_builder_definition = generate_query_builder_definition(table.fields.as_slice())?;

    let mut table_columns = Vec::new();
    for field in &table.fields {
        table_columns.push(generate_table_column(&field)?);
    }

    let table_impl = generate_table_impl(table, table.fields.as_slice())?;
    let queryable = generate_queryable(table, table.fields.as_slice())?;
    let querybuilder_field_impl = generate_querybuilder_field_impl(table.fields.as_slice())?;
    let querybuilder_impl = generate_querybuilder_impl(table.fields.as_slice())?;

    let mod_name = &table.mod_name;

    let result = quote! {
        pub mod #mod_name {
            #query_builder_definition

            #(
                #table_columns
            )*

            #table_impl
            #queryable
            #(
                #querybuilder_field_impl
            )*
            #querybuilder_impl
        }
    };

    Ok(result.into())
}

pub fn generate_query_builder_definition(fields: &[Field]) -> Result<TokenStream2> {
    let mut column_parameters = Vec::with_capacity(fields.len());
    let mut querybuilder_fields = Vec::with_capacity(fields.len());

    for field in fields {
        let field_name = &field.name;
        let type_name = &field.name_upper_t;

        column_parameters.push(type_name);
        querybuilder_fields.push(quote! {
            #[allow(unused)]
            #field_name: #type_name
        });
    }

    Ok(quote! {
        #[allow(unused)]
        pub struct QueryBuilder<#(#column_parameters,)*> {
            #(#querybuilder_fields,)*

            db: orm::InnerContext,
        }
    })
}

pub fn generate_table_column(field: &Field) -> Result<TokenStream2> {
    let name_string = &field.name_string;
    let name = &field.name_upper;
    let code_type = &field.code_type;
    let db_type = &field.db_type;
    let db_type_attributes = &field.db_type_attributes;

    Ok(quote! {
        #[allow(unused)]
        pub struct #name;
        impl orm::Column for #name {
            type Type = #code_type;
            fn name() -> &'static str {
                #name_string
            }
            fn db_type() -> &'static orm::ColumnType {
                use orm::GetColumnType;
                #db_type::get_column_type()
            }
            fn db_type_attributes() -> &'static [&'static orm::ColumnAttribute] {
                &[
                    #(&orm::#db_type_attributes,)*
                ]
            }
        }
    })
}

pub fn generate_table_impl(obj: &Table, fields: &[Field]) -> Result<TokenStream2> {
    let database_name = &obj.database_table_name;
    let ident = &obj.name;

    let mut updater_columns = Vec::with_capacity(fields.len());
    let mut query_builder_empty_params = Vec::with_capacity(fields.len());
    let mut from_reader_columns = Vec::with_capacity(fields.len());

    for (index, field) in fields.iter().enumerate() {
        let ident = &field.name;
        let upper_ident = &field.name_upper;

        updater_columns.push(quote! {
            .column(#upper_ident)
        });
        query_builder_empty_params.push(quote! { () });
        from_reader_columns.push(quote! {
            #ident: row.get_opt(#index).unwrap()?
        });
    }

    Ok(quote! {
        impl orm::Table for super::#ident {
            type QueryBuilder = QueryBuilder<#(#query_builder_empty_params),*>;

            fn table_name() -> &'static str {
                #database_name
            }

            fn update_database_schema<T: orm::Connection>(updater: &mut orm::TableUpdater<T>) -> orm::Result<()> {
                updater.table(#database_name)
                    #(
                        #updater_columns
                    )*
                    .build()
            }

            fn from_reader(row: &postgres::rows::Row) -> orm::Result<super::#ident> {
                Ok(super::#ident {
                    #(
                        #from_reader_columns,
                    )*
                })
            }
        }
    })
}

fn generate_queryable(obj: &Table, fields: &[Field]) -> Result<TokenStream2> {
    let ident = &obj.name;

    let mut generics = Vec::with_capacity(fields.len());
    let mut t_generics = Vec::with_capacity(fields.len());
    let mut field_filters = Vec::with_capacity(fields.len());

    for field in fields {
        let ident = &field.name;
        let upper_ident = &field.name_upper;

        generics.push(upper_ident);
        t_generics.push(&field.name_upper_t);

        field_filters.push(quote! {
            if let Some((format, val)) = self.#ident.as_query_filter(values.len()) {
                query += &format!(
                    " {} \"{}\"{}",
                    if values.is_empty() { "WHERE" } else { "AND" },
                    <#upper_ident as orm::Column>::name(),
                    format
                );
                values.push(val);
            }
        });
    }

    let t_generics_2 = t_generics.clone();
    let t_generics_3 = t_generics.clone();

    Ok(quote! {
        impl<#(#t_generics),*> orm::Queryable<super::#ident> for QueryBuilder<#(#t_generics_2),*>
        where
            #(
                #t_generics_3: orm::AsQueryFilter,
            )*{
            fn get_results(&self) -> orm::Result<Vec<super::#ident>> {
                let fields = vec![
                    #(<#generics as orm::Column>::name(), )*
                ].join(", ");
                let mut query = format!("SELECT {} FROM \"{}\"", fields, <super::#ident as orm::Table>::table_name());
                let mut values = Vec::new();

                #(#field_filters)*

                let conn = self.db.pool.get()?;

                let rows = conn.query(&query, values.as_slice())?;
                let mut results = Vec::with_capacity(rows.len());
                for row in rows.iter() {
                    results.push(<super::#ident as orm::Table>::from_reader(&row)?);
                }
                Ok(results)
            }
        }
    })
}

fn generate_querybuilder_field_impl(fields: &[Field]) -> Result<Vec<TokenStream2>> {
    let mut result = Vec::new();
    for (current_field_index, current_field) in fields.iter().enumerate() {
        let ident = &current_field.name;
        let upper_ident = &current_field.name_upper;

        let mut other_generics = Vec::with_capacity(fields.len());
        let mut unit_generics = Vec::with_capacity(fields.len());
        let mut generics_with_t = Vec::with_capacity(fields.len());
        let mut field_assigns = Vec::with_capacity(fields.len());

        for (other_field_index, other_field) in fields.iter().enumerate() {
            let other_ident = &other_field.name;
            let other_upper_t = &other_field.name_upper_t;

            if other_field_index == current_field_index {
                // ignore other_generics
                unit_generics.push(quote! { () });
                generics_with_t.push(quote! { T });
                field_assigns.push(quote! {
                    #other_ident: val
                });
            } else {
                other_generics.push(quote! { #other_upper_t });
                unit_generics.push(quote! { #other_upper_t });
                generics_with_t.push(quote! { #other_upper_t });
                field_assigns.push(quote! {
                    #other_ident: self.#other_ident
                });
            }
        }

        let other_generics_2 = other_generics.clone();
        let unit_generics_2 = unit_generics.clone();
        let upper_ident_2 = upper_ident.clone();

        result.push(quote! {
            impl<#(#other_generics),*> QueryBuilder<#(#unit_generics),*> {
                pub fn #ident(self) -> orm::Expression<Self, #upper_ident> {
                    orm::Expression::new(self)
                }
            }

            impl<T, #(#other_generics_2),*> orm::ExpressionNext<#upper_ident_2, T> for QueryBuilder<#(#unit_generics_2),*> {
                type Result = QueryBuilder<#(#generics_with_t),*>;
                fn next(self, val: T) -> Self::Result {
                    QueryBuilder {
                        #(#field_assigns, )*
                        db: self.db
                    }
                }
            }
        })
    }

    Ok(result)
}

fn generate_querybuilder_impl(fields: &[Field]) -> Result<TokenStream2> {
    let mut unit_generics = Vec::with_capacity(fields.len());
    let mut field_constructors = Vec::with_capacity(fields.len());
    for field in fields {
        let ident = &field.name;

        unit_generics.push(quote! { () });
        field_constructors.push(quote! {
            #ident: ()
        });
    }

    Ok(quote! {
        impl orm::QueryBuilder for QueryBuilder<#(#unit_generics),*> {
            fn new(inner: orm::InnerContext) -> Self {
                QueryBuilder {
                    #(#field_constructors,)*
                    db: inner
                }
            }
        }
    })
}
