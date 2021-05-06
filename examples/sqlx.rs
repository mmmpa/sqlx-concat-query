#![allow(warnings)]

use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::{ConnectOptions, MySql, Pool};
use sqlx_concat_query::{concat_query, concat_query_as};

#[derive(Debug)]
struct Feed {
    title: String,
}

macro_rules! feed_information {
    ($additional_statement:tt, $($binds:expr),* $(,)?) => {
        concat_query_as!(
            Feed,
            "select title from feeds",
            $additional_statement,
            "ORDER BY title ASC"
            $($binds,)*
        )
    };
}

#[tokio::main]
async fn main() {
    let tx = build_async_pool().await.unwrap();

    let one = feed_information!("where title = ? AND title = ?", 1, 2)
        .fetch_one(&tx)
        .await;

    let all = feed_information!("where created_at > '1999-08-31'",)
        .fetch_all(&tx)
        .await;

    let re = concat_query!(
        "select title from feeds ",
        "where title = ? AND title = ?",
        1,
        2,
    )
    .fetch_one(&tx)
    .await;

    let re = concat_query!("select title", "from feeds")
        .fetch_one(&tx)
        .await;

    let re = concat_query_as!(
        Feed,
        "select title from feeds ",
        "where title = ? AND title = ?",
        1,
        2
    )
    .fetch_one(&tx)
    .await;

    let re = concat_query_as!(Feed, "select title", "from feeds")
        .fetch_one(&tx)
        .await;
}

pub async fn build_async_pool() -> Result<Pool<MySql>, sqlx::Error> {
    MySqlPoolOptions::new().connect(env!("DATABASE_URL")).await
}
