use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Feedback {
    pub id: String,
    pub login: String,
    pub name: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeedbackData {
    pub feedback: Feedback,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserData {
    pub login: String,
    pub password: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeedbackResponse {
    pub status: String,
    pub data: FeedbackData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeedbackListResponse {
    pub status: String,
    pub users: Vec<Feedback>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}
