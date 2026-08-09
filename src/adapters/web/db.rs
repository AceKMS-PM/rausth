use crate::core::identity::User;
use crate::core::repository::{RepositoryError, UserRepository};
//use sqlx::Error;
use sqlx::SqlitePool;

pub struct SqliteUserRepository {
    pool: SqlitePool,
}

impl SqliteUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl UserRepository for SqliteUserRepository {
    async fn find_by_email(&self, email: &str) -> Result<User, RepositoryError> {
        let result = sqlx::query_as!(User,r#"SELECT id as "id!: _" , email, password_hashed, created_at as "created_at: _" from users WHERE email = ? "#, email)
            .fetch_optional(&self.pool)
            .await.map_err(|e| RepositoryError::Database(e.to_string()))?;

        result.ok_or(RepositoryError::NotFound)
    }

    async fn save(&self, user: &User) -> Result<(), RepositoryError> {
        let result = sqlx::query!(
            r#"INSERT INTO users (id , email , password_hashed, created_at) VALUES (?,?,?,?)"#,
            user.id,
            user.email,
            user.password_hashed,
            user.created_at
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                Err(RepositoryError::AlreadyExists)
            }
            Err(e) => Err(RepositoryError::Database(e.to_string())),
        }
    }
}
