use crate::json::JsonResponse;
use argon2_async::verify;
use axum::extract::Json;

use crate::types::{Error, LoginData, SessionData};

/// POST /login
/// Creates a new login session given a user's email and password.
pub async fn login(
    Json(LoginData { email, password }): Json<LoginData>,
) -> Result<JsonResponse<SessionData>, JsonResponse<Error>> {
    let db = get_database!();

    let user = sqlx::query!("SELECT id, password, permissions FROM users WHERE email = $1", email,)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| {
            (
                404,
                Error {
                    message: "User not found".to_string(),
                },
            )
        })?;

    if !verify(password, user.password).await.map_err(|_| {
        (
            500,
            Error {
                message: "Could not validate password".to_string(),
            },
        )
    })? {
        return Err(JsonResponse::new(
            401,
            Error {
                message: "Invalid password".to_string(),
            },
        ));
    }

    let token = sqlx::query!(
        "INSERT INTO auth_sessions (user_id, token, permissions) VALUES ($1, $2, $3) RETURNING token",
        user.id as i64,
        crate::auth::generate_token(user.id as u64),
        user.permissions,
    )
    .fetch_one(db)
    .await?
    .token;

    Ok(JsonResponse::new(
        201,
        SessionData {
            user_id: user.id as u64,
            token,
        },
    ))
}
