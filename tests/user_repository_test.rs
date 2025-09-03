#[cfg(test)]
mod tests {
    use afaf_rest_rust::{config::Config, core::domain::users::repository::UserRepository};
    use sqlx::PgPool;
    use uuid::Uuid;

    async fn setup() -> PgPool {
        let config = Config::from_env();
        PgPool::connect(&config.database_url)
            .await
            .expect("Failed to connect to database")
    }

    #[tokio::test]
    async fn test_create_user() {
        let pool = setup().await;
        let repo = UserRepository { pool: &pool };

        let name = "Test User";
        let email = format!("test_{}@example.com", Uuid::new_v4());

        let result = repo.create_user(name, &email).await;
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.name, name);
        assert_eq!(user.email, email);
    }

    #[tokio::test]
    async fn test_find_all_users() {
        let pool = setup().await;
        let repo = UserRepository { pool: &pool };

        // Create a test user first
        let name = "Test User for Find";
        let email = format!("test_find_{}@example.com", Uuid::new_v4());
        let create_result = repo.create_user(name, &email).await;
        assert!(create_result.is_ok());

        // Test find_all
        let result = repo.find_all().await;
        assert!(result.is_ok());

        let users = result.unwrap();
        assert!(!users.is_empty());
        assert!(users.iter().any(|u| u.email == email));
    }

    #[tokio::test]
    async fn test_create_duplicate_email() {
        let pool = setup().await;
        let repo = UserRepository { pool: &pool };

        let name = "Test User";
        let email = format!("test_duplicate_{}@example.com", Uuid::new_v4());

        // Create first user
        let first_result = repo.create_user(name, &email).await;
        assert!(first_result.is_ok());

        // Try to create second user with same email
        let second_result = repo.create_user("Another User", &email).await;
        assert!(second_result.is_err());

        match second_result {
            Err(sqlx::Error::Database(db_err)) => {
                assert_eq!(db_err.constraint(), Some("users_email_key"));
            }
            _ => panic!("Expected database constraint error"),
        }
    }
}
