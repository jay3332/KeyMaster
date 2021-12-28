use crate::json::JsonResponse;
use argon2_async::hash;
use axum::extract::Json;
use rand::{thread_rng, Rng};

use crate::auth::generate_snowflake;
use crate::types::{CreateUserData, Error, Success};

/// POST /users
pub async fn create_user(
    Json(CreateUserData { name, email, password }): Json<CreateUserData>
) -> Result<JsonResponse<Success>, JsonResponse<Error>> {
    let db = get_database!();

    let snowflake = generate_snowflake();
    sqlx::query!(
        "INSERT INTO users (id, name, discriminator, email, password) VALUES ($1, $2, $3, $4, $5)", 
        snowflake as i64,
        name,
        {
            let existing: Vec<i16> =
                sqlx::query!("SELECT discriminator FROM users WHERE name = $1", name)
                    .fetch_all(db)
                    .await
                    .map(|r| r.into_iter().map(|x| x.discriminator).collect())?;

            let available = (1..=9999)
                .filter(|x| !existing.contains(x))
                .collect::<Vec<_>>();

            *available
                .get(thread_rng().gen_range(0..available.len()))
                .ok_or_else(|| (409, Error {
                    message: "This username is already taken.".to_string()
                }))?
        },
        email,
        hash(&password).await.map_err(|e| (500, Error {
            message: format!("Could not hash password: {:?}", e),
        }))?,
    )
        .execute(db)
        .await?;

    Ok(JsonResponse::new(
        201,
        Success {
            message: "User registered successfully".to_string(),
            id: Some(snowflake as u64),
        },
    ))
}
