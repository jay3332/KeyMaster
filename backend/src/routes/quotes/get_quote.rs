use crate::json::JsonResponse;
use crate::types::{Error, Quote};

use axum::extract::Path;

/// GET /quotes/:id
pub async fn get_quote(Path(id): Path<u32>) -> Result<JsonResponse<Quote>, JsonResponse<Error>> {
    let db = get_database!();
    let quote = sqlx::query!("SELECT * FROM quotes WHERE id = $1", id as i32)
        .fetch_optional(db)
        .await?
        .ok_or(JsonResponse::new(
            404,
            Error {
                message: "Quote not found".to_string(),
            },
        ))?;

    Ok(JsonResponse::new(
        200,
        Quote {
            id,
            author_id: quote.author_id.map(|o| o as u64),
            content: quote.content.clone(),
            author: quote.author.map(|o| o.clone()),
        },
    ))
}
