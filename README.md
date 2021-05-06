# sqlx-concat-query

This builds a string literal to reuse huge raw querirs for sqlx's macros.

This provides `concat_query!` and `concat_query_as!`. They concat serial string literals that are passed as queries.

```rust
concat_query!(
    "SELECT title FROM feeds ",        //  ┬─ concat
    "WHERE title = ? AND status = ?",  //  ┘
    title,
    status
)

concat_query_as!(
    Feed,
    "SELECT title FROM feeds ",       //  ┐
    "WHERE title = ? AND status = ?", //  ┼─ concat
    "ORDER BY title ASC".             //  ┘
    title,
    status
)
```

# Usage

Define a macro to reuse queries.

```rust
macro_rules! feed_information {
    ($additional_statement:tt, $($binds:expr),* $(,)?) => {
        concat_query_as!(
            Feed,
            "
              TOO
              LOND
              SELECT
              QUERY
            ",
            $additional_statement,
            "ORDER BY title ASC"
            $($binds,)*
        )
    };
}
```

And use the defined macro.

```rust
let one = feed_information!("where title = ? AND status = ?", title, status)
    .fetch_one(&tx)
    .await?;

let all = feed_information!("where created_at > '1999-08-31'",)
    .fetch_all(&tx)
    .await?;
```
