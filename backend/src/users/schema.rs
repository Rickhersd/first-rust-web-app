use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Serialize, Deserialize, Debug)]
pub struct SigUpUserSchema {
    pub login: String,
    pub password: String,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUserSchema {
    pub login: String,
    pub password: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct UserMeResponse {
    pub id: String,
    pub name: String,
    pub role: String,
}

#[derive(Deserialize, FromRow, Serialize, Debug)]
pub struct AllUsersResponse {
    pub id: String,
    pub name: String,
    pub login: String,
    pub role: String
}




