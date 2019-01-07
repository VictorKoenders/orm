use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Field, ItemStruct, Result, Type};

pub fn generate_implementation(obj: &ItemStruct) -> Result<TokenStream> {
    let mod_name = Ident::new(&obj.ident.to_string().to_lowercase(), Span::call_site());
    let fields = obj.fields.iter().collect::<Vec<_>>();
    let query_builder_definition = generate_query_builder_definition(fields.as_slice())?;

    let mut table_columns = Vec::new();
    for field in &fields {
        table_columns.push(generate_table_column(&field, &fields)?);
    }

    let table_impl = generate_table_impl(&obj, fields.as_slice())?;
    let queryable = generate_queryable(&obj, fields.as_slice())?;
    let querybuilder_field_impl = generate_querybuilder_field_impl(fields.as_slice())?;
    let querybuilder_impl = generate_querybuilder_impl(fields.as_slice())?;

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

pub fn generate_query_builder_definition(fields: &[&Field]) -> Result<TokenStream2> {
    let mut column_parameters = Vec::with_capacity(fields.len());
    let mut querybuilder_fields = Vec::with_capacity(fields.len());

    for field in fields {
        let ident = match field.ident.as_ref() {
            Some(i) => i.to_string(),
            None => return Err(syn::Error::new(field.span(), "Fields need to be named")),
        }
        .to_string();

        let field_name = Ident::new(&ident, Span::call_site());
        let type_name = Ident::new(&ident.to_uppercase(), Span::call_site());

        column_parameters.push(type_name.clone());
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

pub fn generate_table_column(field: &Field, fields: &[&Field]) -> Result<TokenStream2> {
    let name_string = match field.ident.as_ref() {
        Some(i) => i.to_string(),
        None => return Err(syn::Error::new(field.span(), "Fields need to be named")),
    };
    let name = Ident::new(&name_string.to_uppercase(), Span::call_site());
    let type_ = &field.ty;
    let mut db_type = get_database_type_from_field_type(type_)?;
    if is_identifier(field, fields) {
        if db_type != "UUID" {
            db_type = String::from("SERIAL PRIMARY KEY");
        } else {
            db_type += " PRIMARY KEY DEFAULT(uuid_generate_v4())";
        }
    }

    Ok(quote! {
        #[allow(unused)]
        pub struct #name;
        impl orm::Column for #name {
            type Type = #type_;
            fn name() -> &'static str {
                #name_string
            }
            fn db_type() -> &'static str {
                #db_type
            }
        }
    })
}

pub fn get_database_type_from_field_type(t: &Type) -> Result<String> {
    match t {
        Type::Path(p) => {
            if let Some(segment) = p.path.segments.iter().next() {
                let ident_str: &str = &segment.ident.to_string();
                match ident_str {
                    "i32" | "u32" => return Ok(String::from("INT")),
                    "String" => return Ok(String::from("TEXT")),
                    _ => return Err(syn::Error::new(t.span(), format!("Could not determine database type for {:?}", ident_str)))
                }
            }
        }
        _ => {}
    }
    Err(syn::Error::new(t.span(), format!("Could not determine database type for {:?}", t)))
}

pub fn is_identifier(field: &Field, _fields: &[&Field]) -> bool {
    if let Some(i) = &field.ident {
        i == "id"
    } else {
        false
    }
}

pub fn generate_table_impl(obj: &ItemStruct, fields: &[&Field]) -> Result<TokenStream2> {
    let ident = &obj.ident;
    let database_name = ident.to_string().to_lowercase();

    let mut updater_columns = Vec::with_capacity(fields.len());
    let mut query_builder_empty_params = Vec::with_capacity(fields.len());
    let mut from_reader_columns = Vec::with_capacity(fields.len());

    for (index, field) in fields.iter().enumerate() {
        let ident = match field.ident.as_ref() {
            Some(i) => i,
            None => return Err(syn::Error::new(field.span(), "Fields need to be named")),
        };

        let upper_ident = Ident::new(&ident.to_string().to_uppercase(), Span::call_site());

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

            fn update_database_schema(updater: &mut orm::TableUpdater) -> orm::Result<()> {
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

fn generate_queryable(obj: &ItemStruct, fields: &[&Field]) -> Result<TokenStream2> {
    let ident = &obj.ident;

    let mut generics = Vec::with_capacity(fields.len());
    let mut t_generics = Vec::with_capacity(fields.len());
    let mut field_filters = Vec::with_capacity(fields.len());

    for field in fields {
        let ident = match field.ident.as_ref() {
            Some(i) => i,
            None => return Err(syn::Error::new(field.span(), "Fields need to be named")),
        };
        let upper_ident = Ident::new(&ident.to_string().to_uppercase(), Span::call_site());
        let upper_ident_t = Ident::new(
            &format!("T{}", ident.to_string().to_uppercase()),
            Span::call_site(),
        );

        generics.push(upper_ident.clone());
        t_generics.push(upper_ident_t.clone());

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

fn generate_querybuilder_field_impl(fields: &[&Field]) -> Result<Vec<TokenStream2>> {
    let mut result = Vec::new();
    for (current_field_index, current_field) in fields.iter().enumerate() {
        let ident = current_field.ident.clone().unwrap();
        let upper_ident = Ident::new(&ident.to_string().to_uppercase(), Span::call_site());

        let mut other_generics = Vec::new();
        let mut unit_generics = Vec::new();
        let mut generics_with_t = Vec::new();
        let mut field_assigns = Vec::new();

        for (other_field_index, other_field) in fields.iter().enumerate() {
            let other_ident = other_field.ident.clone().unwrap();
            let other_upper_t = Ident::new(
                &format!("T{}", other_ident.to_string().to_uppercase()),
                Span::call_site(),
            );

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

fn generate_querybuilder_impl(fields: &[&Field]) -> Result<TokenStream2> {
    let mut unit_generics = Vec::new();
    let mut field_constructors = Vec::new();
    for field in fields {
        unit_generics.push(quote! { () });
        let ident = field.ident.as_ref().unwrap();
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
