use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;

use crate::json::WebSocketJsonResponse;
use crate::routes::{get_auth, quotes::get_random_quote, ExtractJson};
use crate::types::{WebSocketInboundEvent, WebSocketIntent, WebSocketOutboundEvent};

pub async fn practice_quote(ws_upgrade: WebSocketUpgrade) -> impl IntoResponse {
    ws_upgrade.on_upgrade(async move |mut ws: WebSocket| {
        while let Some(message) = ws.recv().await {
            let message = if let Ok(message) = message {
                match message.extract_json() {
                    Ok(message) => message,
                    Err(_) => return,
                }
            } else {
                return;
            };

            // TODO: disconnect rather than return from here
            match message {
                WebSocketInboundEvent::Identify { token, intent }
                    if matches!(intent, WebSocketIntent::PracticeQuote) =>
                {
                    let auth = match get_auth(token).await {
                        Ok(auth) => auth,
                        Err(_) => return,
                    };

                    let quote = match get_random_quote(auth).await {
                        Ok(response) => response.json,
                        Err(_) => return,
                    };

                    match WebSocketJsonResponse(WebSocketOutboundEvent::PracticeQuoteReady {
                        quote,
                    })
                    .send(&mut ws)
                    .await
                    {
                        Ok(_) => (),
                        Err(_) => return,
                    };
                }
                _ => continue,
            }
        }
    })
}
