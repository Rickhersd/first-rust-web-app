use crate::jwt_auth;

use crate::model::TokenClaims;
use crate::users::{
    schema::{AllUsersResponse, LoginUserSchema, SigUpUserSchema, UserMeResponse},
    UserModel,
};
use crate::AppState;

use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpResponse, Responder,
};
use actix_web::{HttpMessage, HttpRequest};
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

#[post("/auth/signup")]
async fn signup(body: web::Json<SigUpUserSchema>, data: web::Data<AppState>) -> impl Responder {
    let user_exists = UserModel::check_if_exists(&body.login, &data).await;

    if user_exists == true {
        HttpResponse::Conflict()
            .json(serde_json::json!({"status": "error","message": "The user already exists"}));
    }

    let response: Result<UserModel, sqlx::Error> = UserModel::create(body, data).await;

    match response {
        Ok(_) => HttpResponse::Ok().into(),
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

#[post("/auth/login")]
async fn login(body: web::Json<LoginUserSchema>, data: web::Data<AppState>) -> impl Responder {
    println!("🚀 Server started successfully");

    let query_result = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE login = $1",
        body.login
    )
    .fetch_optional(&data.db)
    .await
    .unwrap();

    let is_valid = query_result.to_owned().map_or(false, |user| {
        let parsed_hash = PasswordHash::new(&user.password).unwrap();
        Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true)
    });

    if !is_valid {
        return HttpResponse::BadRequest()
            .json(json!({"status": "fail", "message": "Invalid email or password"}));
    }

    let user = query_result.unwrap();

    let now = Utc::now();
    let iat: usize = now.timestamp() as usize;
    let exp: usize = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie: actix_web::cookie::Cookie<'_> = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(60 * 60, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success", "token": token}))
}

#[get("/users/me")]
async fn get_me_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    let user = sqlx::query_as!(
        UserMeResponse,
        "SELECT 
            u.id,
            u.name,
            r.code as role
        FROM users as u 
        JOIN roles as r
            on r.id = u.role_id
        WHERE 
            u.id = $1",
        user_id
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let json_response: serde_json::Value = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": user
        })
    });

    HttpResponse::Ok().json(json_response)
}

#[get("/users")]
async fn get_all_users(data: web::Data<AppState>) -> impl Responder {
    let users_result = sqlx::query_as!(
        AllUsersResponse,
        "SELECT 
            u.id,
            u.name,
            u.login,
            r.code as role
        FROM  users as u
        JOIN roles as r
            on r.id = u.role_id 
        ORDER by id"
    )
    .fetch_all(&data.db)
    .await;

    if users_result.is_err() {
        let message = "Something bad happened while fetching all feedback items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let users_result = users_result.unwrap();
    let json_response = serde_json::json!({
        "status": "success",
        "users": users_result
    });

    HttpResponse::Ok().json(json_response)
}

pub fn init_routes_users(config: &mut web::ServiceConfig) {
    config.service(login);
    config.service(signup);
    config.service(get_me_handler);
    config.service(get_all_users);
}
