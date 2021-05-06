use proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::token::Comma;
use syn::{parse_macro_input, LitStr, Type};

fn join_query(input: ParseStream) -> Result<String> {
    let mut query = String::new();

    loop {
        if input.peek(Comma) {
            input.parse::<Comma>()?;
        }

        if input.peek(LitStr) {
            let s: LitStr = input.parse()?;
            query += &s.value();
            query.push(' ');
        } else {
            break;
        }
    }

    Ok(query)
}

struct Query {
    query: String,
    rest: TokenStream2,
}

impl Parse for Query {
    fn parse(input: ParseStream) -> Result<Self> {
        let query = join_query(input)?;

        Ok(Query {
            query,
            rest: input.parse()?,
        })
    }
}

struct QueryAs {
    record: Type,
    query: String,
    rest: TokenStream2,
}

impl Parse for QueryAs {
    fn parse(input: ParseStream) -> Result<Self> {
        let record = input.parse()?;
        input.parse::<Comma>()?;

        let query = join_query(input)?;

        Ok(QueryAs {
            record,
            query,
            rest: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn concat_query(input: TokenStream) -> TokenStream {
    let Query { query, rest } = parse_macro_input!(input as Query);

    let tokens = quote! {
        sqlx::query!(#query, #rest)
    };

    tokens.into()
}

#[proc_macro]
pub fn concat_query_as(input: TokenStream) -> TokenStream {
    let QueryAs {
        record,
        query,
        rest,
    } = parse_macro_input!(input as QueryAs);

    let tokens = quote! {
        sqlx::query_as!(#record, #query, #rest)
    };

    tokens.into()
}
