use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptionsResponse;

#[options("/registration")]
pub async fn cors_registration() -> Json<OptionsResponse> {
    Json(OptionsResponse)
}

#[options("/login")]
pub async fn cors_login() -> Json<OptionsResponse> {
    Json(OptionsResponse)
}

#[options("/refresh")]
pub async fn cors_refresh() -> Json<OptionsResponse> {
    Json(OptionsResponse)
}

#[options("/public/hello")]
pub async fn cors_pub_hello() -> Json<OptionsResponse> {
    Json(OptionsResponse)
}

#[options("/user")]
pub async fn cors_user() -> Json<OptionsResponse> {
    Json(OptionsResponse)
}
#[options("/hello")]
pub async fn cors_hello() -> Json<OptionsResponse> {
    Json(OptionsResponse)
}
