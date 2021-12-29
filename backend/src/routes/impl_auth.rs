use axum::body::Body;
use axum::extract::{FromRequest, RequestParts};
use http::header::AUTHORIZATION;

use crate::auth::decode_token;
use crate::json::JsonResponse;
use crate::types::{Error, UserPermissionFlags};

pub struct Auth(pub u64, pub UserPermissionFlags);

#[async_trait::async_trait]
impl FromRequest<Body> for Auth {
    type Rejection = JsonResponse<Error>;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let authorization = req
            .headers()
            .ok_or_else(|| {
                (
                    500,
                    Error {
                        message: "Could not properly read headers.".to_string(),
                    },
                )
            })?
            .get(AUTHORIZATION)
            .ok_or_else(|| {
                (
                    400,
                    Error {
                        message: "'Authorization' header is required.".to_string(),
                    },
                )
            })?;

        let token = String::from_utf8(authorization.as_bytes().into()).map_err(|e| {
            (
                400,
                Error {
                    message: format!("'Authorization' header is not valid UTF-8: {}", e),
                },
            )
        })?;

        Ok(get_auth(token).await?)
    }
}

pub async fn get_auth(token: String) -> Result<Auth, JsonResponse<Error>> {
    let user_id = decode_token(token.clone()).ok_or_else(|| {
        (
            500,
            Error {
                message: "Could not decode token.".to_string(),
            },
        )
    })?;

    let db = get_database!();
    let permissions = sqlx::query!(
        "SELECT permissions FROM auth_sessions WHERE token = $1 AND user_id = $2",
        token,
        user_id as i64,
    )
    .fetch_optional(db)
    .await?
    .ok_or_else(|| {
        (
            401,
            Error {
                message: "Received invalid token.".to_string(),
            },
        )
    })?
    .permissions;

    Ok(Auth(
        user_id,
        UserPermissionFlags::from_bits(permissions as u64).unwrap(),
    ))
}
