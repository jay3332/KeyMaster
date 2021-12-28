use crate::json::JsonResponse;
use crate::types::{Error, Quote};

/// GET /quotes
pub async fn get_random_quote() -> Result<JsonResponse<Quote>, JsonResponse<Error>> {
    let db = get_database!();
    let quote = sqlx::query!("SELECT * FROM quotes ORDER BY RANDOM()")
        .fetch_one(db)
        .await?;

    Ok(JsonResponse::new(
        200,
        Quote {
            id: quote.id as u32,
            author_id: quote.author_id.map(|o| o as u64),
            content: quote.content.clone(),
            author: quote.author.map(|o| o.clone()),
        },
    ))
}
