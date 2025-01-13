use crate::core::domain::users::model::User;
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserRepository<'a> {
    pub pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    // Find all users
    pub async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(self.pool)
            .await
    }

    // Create a new user
    pub async fn create_user(&self, name: &str, email: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (id, name, email) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(Uuid::new_v4())  
        .bind(name)
        .bind(email)
        .fetch_one(self.pool)
        .await
    }
}
