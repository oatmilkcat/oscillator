use std::sync::LazyLock;

use axum::{Json, Router, routing::post};
use axum_valid::Valid;
use regex::Regex;
use serde::Deserialize;
use utoipa::openapi::ServerVariableBuilder;
use validator::{Validate, ValidationError};

use crate::{ApiResult, errors::ApiError};

pub fn router() -> Router {
    Router::new()
        .route("/authenticate", post(authenticate))
        .route("/register", post(register))
}

const USERNAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9-_.]*$").unwrap());

fn validate_password(password: &str) -> Result<(), ValidationError> {
    // TODO
    Ok(())
}

#[derive(Debug, Deserialize, Validate)]
struct AuthenticateRequest {
    #[validate(length(min = 1, max = 20), regex(path = *USERNAME_REGEX))]
    username: String,

    #[validate(length(min = 16, max = 512), custom(function = "validate_password"))]
    password: String,
}

async fn authenticate(Valid(body): Valid<Json<AuthenticateRequest>>) -> ApiResult<&'static str> {
    Err(ApiError::Unauthorized)
}

#[derive(Debug, Deserialize, Validate)]
struct RegisterRequest {
    #[validate(length(max = 30))]
    display_name: Option<String>,

    #[validate(email)]
    email: String,

    #[validate(length(min = 1, max = 20), regex(path = *USERNAME_REGEX))]
    username: String,

    #[validate(length(min = 16, max = 512), custom(function = "validate_password"))]
    password: String,
}

async fn register(Valid(body): Valid<Json<RegisterRequest>>) -> ApiResult<&'static str> {
    Err(ApiError::Unauthorized)
}
