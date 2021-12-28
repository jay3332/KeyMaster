use crate::json::JsonResponse;
use crate::routes::Auth;
use crate::types::{Error, User};

/// GET /users/me
pub async fn get_me(Auth(user_id): Auth) -> Result<JsonResponse<User>, JsonResponse<Error>> {
    let db = get_database!();
    let user = sqlx::query!("SELECT * FROM users WHERE id = $1", user_id as i64)
        .fetch_optional(db)
        .await?
        .ok_or(JsonResponse::new(
            404,
            Error {
                message: "User not found".to_string(),
            },
        ))?;

    Ok(JsonResponse::new(
        200,
        User {
            id: user_id,
            name: user.name.clone(),
            discriminator: user.discriminator as u16,
            email: user.email.clone(),
        },
    ))
}
