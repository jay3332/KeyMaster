use crate::json::JsonResponse;
use axum::extract::Path;

use crate::types::{Error, User};

/// GET /users/:id
pub async fn get_user(Path(id): Path<u64>) -> Result<JsonResponse<User>, JsonResponse<Error>> {
    let db = get_database!();
    let user = sqlx::query!("SELECT * FROM users WHERE id = $1", id as i64)
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
            id,
            name: user.name.clone(),
            discriminator: user.discriminator as u16,
        },
    ))
}
