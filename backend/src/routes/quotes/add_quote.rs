use crate::json::JsonResponse;
use axum::extract::Json;

use crate::routes::Auth;
use crate::types::{Error, QuoteData, Success, UserPermissionFlags};

/// POST /quotes
/// Adds a single quote into the database.
pub async fn add_quote(
    Json(QuoteData { content, author }): Json<QuoteData>,
    Auth(_, permissions): Auth,
) -> Result<JsonResponse<Success>, JsonResponse<Error>> {
    permissions.expect_permission(UserPermissionFlags::ADD_QUOTES)?;
    let db = get_database!();

    let response = sqlx::query!(
        "INSERT INTO quotes (content, author) VALUES ($1, $2) RETURNING id",
        content,
        author,
    )
    .fetch_one(db)
    .await?;

    Ok(JsonResponse::new(
        201,
        Success {
            message: "Created quote".to_string(),
            id: Some(response.id as u64),
        },
    ))
}
