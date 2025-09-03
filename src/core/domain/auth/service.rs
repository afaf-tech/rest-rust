use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::core::domain::{
    auth::jwt::JwtService,
    error::{AppError, Result},
    users::model::{AuthResponse, CreateUserRequest, LoginRequest, PublicUser, User, UserRole},
};

pub struct AuthService {
    jwt_service: JwtService,
}

impl AuthService {
    pub fn new(jwt_secret: &str) -> Self {
        Self {
            jwt_service: JwtService::new(jwt_secret),
        }
    }

    pub fn hash_password(&self, password: &str) -> Result<String> {
        hash(password, DEFAULT_COST).map_err(|_| AppError::Internal)
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        verify(password, hash).map_err(|_| AppError::Internal)
    }

    pub async fn register_user(
        &self,
        pool: &PgPool,
        request: CreateUserRequest,
    ) -> Result<AuthResponse> {
        // Validate input
        request.validate()?;

        // Hash password
        let password_hash = self.hash_password(&request.password)?;

        // Set default role if not provided
        let role = request.role.unwrap_or_else(|| UserRole::User.to_string());

        // Validate role
        role.parse::<UserRole>()
            .map_err(|e| AppError::Validation { message: e })?;

        // Create user in database
        let user_id = Uuid::new_v4();
        let now = chrono::Utc::now();

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, name, email, password_hash, role, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(&request.name)
        .bind(&request.email)
        .bind(&password_hash)
        .bind(&role)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await?;

        // Generate token
        let token = self
            .jwt_service
            .generate_token(user.id, &user.email, &user.role)?;

        Ok(AuthResponse {
            token,
            user: user.into(),
        })
    }

    pub async fn login_user(&self, pool: &PgPool, request: LoginRequest) -> Result<AuthResponse> {
        // Validate input
        request.validate()?;

        // Find user by email
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(&request.email)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::Authentication {
                message: "Invalid credentials".to_string(),
            })?;

        // Verify password
        if !self.verify_password(&request.password, &user.password_hash)? {
            return Err(AppError::Authentication {
                message: "Invalid credentials".to_string(),
            });
        }

        // Generate token
        let token = self
            .jwt_service
            .generate_token(user.id, &user.email, &user.role)?;

        Ok(AuthResponse {
            token,
            user: user.into(),
        })
    }

    pub fn verify_token(&self, token: &str) -> Result<crate::core::domain::auth::jwt::Claims> {
        self.jwt_service.verify_token(token)
    }

    pub async fn get_user_by_id(&self, pool: &PgPool, user_id: Uuid) -> Result<PublicUser> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::NotFound {
                resource: "User".to_string(),
            })?;

        Ok(user.into())
    }
}
