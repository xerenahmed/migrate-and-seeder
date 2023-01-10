use fake::{
    faker::{
        internet::raw::SafeEmail,
        lorem::{raw::Paragraph, raw::Sentence},
        name::raw::Name,
    },
    locales::EN,
    Fake,
};
use rand::prelude::SliceRandom;
use sqlx::{
    mysql::{MySqlPoolOptions, MySqlRow},
    MySql, Pool, Row,
};
type MySqlPool = Pool<MySql>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:og-password@localhost:3399/deneme")
        .await?;

    seed_users(&pool, 150).await.expect("Failed to seed users");
    seed_posts(&pool, 60).await.expect("Failed to seed posts");
    Ok(())
}

async fn seed_users(pool: &MySqlPool, count: i32) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM users").execute(&mut tx).await?;

    for _i in 0..count {
        let name = Name(EN).fake::<String>();
        let email = SafeEmail(EN).fake::<String>();
        sqlx::query(
            r#"
        INSERT INTO users (name, email, password)
        VALUES (?, ?, ?)
        "#,
        )
        .bind(name)
        .bind(email)
        .bind("password")
        .execute(&mut tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}

async fn seed_posts(pool: &MySqlPool, count: i32) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM posts").execute(&mut tx).await?;

    let user_ids: Vec<MySqlRow> = sqlx::query(
        r#"
        SELECT id FROM users
        "#,
    )
    .fetch_all(&mut tx)
    .await?;
    let user_ids = user_ids
        .iter()
        .map(|row| row.get::<u64, _>("id"))
        .collect::<Vec<_>>();

    for _i in 0..count {
        let title = Sentence(EN, 3..6).fake::<String>();
        let body = Paragraph(EN, 5..10).fake::<String>();
        sqlx::query(
            r#"
        INSERT INTO posts (title, body, author_id)
        VALUES (?, ?, ?)
        "#,
        )
        .bind(title)
        .bind(body)
        .bind(user_ids.choose(&mut rand::thread_rng()))
        .execute(&mut tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}
