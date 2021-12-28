use crate::json::JsonResponse;
use axum::extract::Json;

use crate::routes::Auth;
use crate::types::{Error, QuotesData, Success, UserPermissionFlags};

/// POST /quotes/bulk
/// Adds multiple quotes into the database.
pub async fn add_quotes(
    Auth(_, permissions): Auth,
    Json(QuotesData { quotes }): Json<QuotesData>,
) -> Result<JsonResponse<Success>, JsonResponse<Error>> {
    permissions.expect_permission(UserPermissionFlags::ADD_QUOTES)?;

    let db = get_database!();
    let count = sqlx::query(
        "INSERT INTO quotes (content, author) SELECT * FROM UNNEST($1, $2) AS _(content, author)",
    )
    .bind(
        quotes
            .iter()
            .map(|q| q.content.clone())
            .collect::<Vec<String>>(),
    )
    .bind(
        quotes
            .iter()
            .map(|q| q.author.clone())
            .collect::<Vec<Option<String>>>(),
    )
    .execute(db)
    .await?
    .rows_affected();

    Ok(JsonResponse::new(
        201,
        Success {
            message: format!("{} quotes added", count),
            id: None,
        },
    ))
}
