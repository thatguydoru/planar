use sqlx::SqlitePool;

use crate::error::AppError;

type Result<T> = std::result::Result<T, AppError>;

pub struct User {
    pub id: i64,
    pub username: String,
}

impl User {
    pub async fn query_by_id(db: &SqlitePool, id: i64) -> Result<Self> {
        let user = sqlx::query_as!(Self, r#"SELECT * FROM users WHERE id=?1 "#, id)
            .fetch_one(db)
            .await?;

        Ok(user)
    }

    // TODO: Check if username is within limit of 255 length.
    pub async fn change_username(db: &SqlitePool, id: i64, username: &str) -> Result<u64> {
        let rows = sqlx::query!(r#"UPDATE users SET username=?1 WHERE id=?2"#, username, id)
            .execute(db)
            .await?
            .rows_affected();

        Ok(rows)
    }

    pub async fn save(mut self, db: &SqlitePool) -> Result<Self> {
        self.id = sqlx::query!(r#"INSERT INTO users (username) VALUES (?1)"#, self.username)
            .execute(db)
            .await?
            .last_insert_rowid();

        Ok(self)
    }
}

pub struct Board {
    pub id: i64,
    pub owner: i64,
    pub title: String,
    pub description: Option<String>,
}

pub struct Column {
    pub id: i64,
    pub owner: i64,
    pub board: i64,
    pub title: String,
    pub description: Option<String>,
}

pub struct Card {
    pub id: i64,
    pub owner: i64,
    pub board: i64,
    pub column: i64,
    pub title: String,
    pub description: Option<String>,
}
