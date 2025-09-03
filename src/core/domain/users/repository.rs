use crate::core::domain::users::model::{User, UserRole};
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserRepository<'a> {
    pub pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    /// Find all users
    pub async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, name, email, password_hash, role, created_at, updated_at FROM users",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Create a new user (legacy method for backward compatibility)
    pub async fn create_user(&self, name: &str, email: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (id, name, email, password_hash, role, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, NOW(), NOW()) RETURNING id, name, email, password_hash, role, created_at, updated_at",
        )
        .bind(Uuid::new_v4())
        .bind(name)
        .bind(email)
        .bind("") // Empty password hash for legacy compatibility
        .bind("user") // Default role
        .fetch_one(self.pool)
        .await
    }

    /// Create a new user with password and role
    pub async fn create_user_with_password(
        &self,
        name: &str,
        email: &str,
        password_hash: &str,
        role: &UserRole,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (id, name, email, password_hash, role, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, NOW(), NOW()) RETURNING id, name, email, password_hash, role, created_at, updated_at",
        )
        .bind(Uuid::new_v4())
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .bind(role.to_string())
        .fetch_one(self.pool)
        .await
    }

    /// Find user by email
    pub async fn find_by_email(&self, email: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, name, email, password_hash, role, created_at, updated_at FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_one(self.pool)
        .await
    }

    /// Find user by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, name, email, password_hash, role, created_at, updated_at FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_one(self.pool)
        .await
    }

    /// Update user password
    pub async fn update_password(
        &self,
        id: Uuid,
        password_hash: &str,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2 RETURNING id, name, email, password_hash, role, created_at, updated_at",
        )
        .bind(password_hash)
        .bind(id)
        .fetch_one(self.pool)
        .await
    }

    /// Update user profile
    pub async fn update_user(
        &self,
        id: Uuid,
        name: Option<&str>,
        email: Option<&str>,
    ) -> Result<User, sqlx::Error> {
        let mut query = "UPDATE users SET updated_at = NOW()".to_string();
        let mut params: Vec<&str> = vec![];
        let mut param_count = 1;

        if let Some(name) = name {
            query.push_str(&format!(", name = ${}", param_count));
            params.push(name);
            param_count += 1;
        }

        if let Some(email) = email {
            query.push_str(&format!(", email = ${}", param_count));
            params.push(email);
            param_count += 1;
        }

        query.push_str(&format!(" WHERE id = ${} RETURNING id, name, email, password_hash, role, created_at, updated_at", param_count));

        let mut sql_query = sqlx::query_as::<_, User>(&query);

        for param in params {
            sql_query = sql_query.bind(param);
        }

        sql_query = sql_query.bind(id);

        sql_query.fetch_one(self.pool).await
    }

    /// Update user role (admin only)
    pub async fn update_user_role(&self, id: Uuid, role: &UserRole) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "UPDATE users SET role = $1, updated_at = NOW() WHERE id = $2 RETURNING id, name, email, password_hash, role, created_at, updated_at",
        )
        .bind(role.to_string())
        .bind(id)
        .fetch_one(self.pool)
        .await
    }

    /// Delete user
    pub async fn delete_user(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    /// Find users by role
    pub async fn find_by_role(&self, role: &UserRole) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, name, email, password_hash, role, created_at, updated_at FROM users WHERE role = $1",
        )
        .bind(role.to_string())
        .fetch_all(self.pool)
        .await
    }

    /// Check if email exists
    pub async fn email_exists(&self, email: &str) -> Result<bool, sqlx::Error> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(self.pool)
            .await?;
        Ok(count.0 > 0)
    }
}
