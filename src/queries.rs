use sqlx::SqlitePool;

use crate::error::ModelError;

pub type Result<T> = std::result::Result<T, ModelError>;

pub struct User {
    pub id: i64,
    pub username: String,
}

impl User {
    pub async fn query_by_id(pool: &SqlitePool, id: i64) -> Result<Self> {
        let user = sqlx::query_as!(Self, r#"SELECT id, username FROM users WHERE id=?1 "#, id)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    pub async fn change_username(pool: &SqlitePool, id: i64, username: &str) -> Result<()> {
        if username.len() > 255 {
            return Err(ModelError::value(
                "Username must be at most 255 characters long.",
            ));
        }

        sqlx::query!(r#"UPDATE users SET username=?1 WHERE id=?2"#, username, id)
            .execute(pool)
            .await?
            .rows_affected();

        Ok(())
    }

    pub async fn save(mut self, pool: &SqlitePool) -> Result<Self> {
        self.id = sqlx::query!(r#"INSERT INTO users(username) VALUES (?1)"#, self.username)
            .execute(pool)
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
