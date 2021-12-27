use crate::json::JsonResponse;
use axum::extract::Json;

use crate::types::{Error, QuoteData, Success};

/// POST /quotes
/// Adds a single quote into the database.
pub async fn add_quote(
    Json(QuoteData { content, author }): Json<QuoteData>,
) -> Result<JsonResponse<Success>, JsonResponse<Error>> {
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
