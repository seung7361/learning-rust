use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub email: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Deserialize)]
pub struct SignInData {
    pub email: String,
    pub password: String,
}

pub async fn sign_in(
    Json(user_data): Json<SignInData>,
) -> Result<Json<String>, StatusCode> {
    let user = match retrieve_user_by_email(&user_data.email) {
        Some(user) => user,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    if !verify_password(&user_data.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap() {
            return Err(StatusCode::UNAUTHORIZED);
    }
    
    let token = encode_jwt(user.email)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    Ok(Json(token))
}

#[derive(Clone)]
pub struct CurrentUser {
    pub email: String,
    pub name: String,
    pub password_hash: String,
}

fn retrieve_user_by_email(email: &str) -> Option<CurrentUser> {
    let current_user: CurrentUser = CurrentUser {
        email: "myemail@gmail.com".to_string(),
        name: "name".to_string(),
        password_hash: "$2b$12$Gwf0uvxH3L7JLfo0CC/NCOoijK2vQ/wbgP.LeNup8vj6gg31IiFkm".to_string()
    };

    Some(current_user)
}