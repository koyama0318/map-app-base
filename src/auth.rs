use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::Extension;
use jsonwebtoken::{
    decode,
    Algorithm,
    DecodingKey,
    Validation,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn jwt_middleware(
    Extension(secret): Extension<String>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    if let Some(authen_header) = req.headers().get("Authorization") {
        if let Ok(authen_str) = authen_header.to_str() {
            if authen_str.starts_with("Bearer ") {
                let token = &authen_str[7..];
                let validation = Validation::new(Algorithm::HS256);
                let token_data = decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret(secret.as_ref()),
                    &validation,
                );

                if token_data.is_ok() {
                    return Ok(next.run(req).await);
                }
            }
        }
    }

    Err(axum::http::StatusCode::UNAUTHORIZED)
}
