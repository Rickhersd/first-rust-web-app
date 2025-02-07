use hex;
use sha2::{Digest, Sha256};

use crate::users::schema::SigUpUserSchema;
use actix_web::web;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use uuid::Uuid;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use crate::AppState;

#[derive(Debug, FromRow, Clone, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserModel {
    pub id: Uuid,
    pub name: String,
    pub login: String,
    pub password: String,
    pub role_id: i32,

    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl UserModel {
    pub async fn create(
        user: web::Json<SigUpUserSchema>,
        data: web::Data<AppState>,
    ) -> Result<Self, sqlx::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(user.password.as_bytes(), &salt)
            .expect("Error while hashing password")
            .to_string();

        let query_result: Result<UserModel, sqlx::Error> = sqlx::query_as!(
            UserModel,
            "INSERT INTO users (name, login, password) VALUES ($1, $2, $3) RETURNING *",
            user.name.to_string(),
            user.login.to_string(),
            hashed_password,
        )
        .fetch_one(&data.db)
        .await;

        return query_result;
    }

    pub async fn check_if_exists(login: &String, data: &web::Data<AppState>) -> bool {
        let exists: bool = sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE login = $1)")
            .bind(login.to_owned())
            .fetch_one(&data.db)
            .await
            .unwrap()
            .get(0);

        exists
    }
}
