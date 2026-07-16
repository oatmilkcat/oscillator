pub mod docs;
pub mod errors;
pub mod handlers;

pub type ApiResult<T> = Result<T, errors::ApiError>;
