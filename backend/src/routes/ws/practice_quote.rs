use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;

use std::time::Instant;

use crate::json::WebSocketJsonResponse;
use crate::routes::{Auth, get_auth, quotes::get_random_quote, ExtractJson};
use crate::types::{WebSocketInboundEvent, WebSocketIntent, WebSocketOutboundEvent};

/// GET /ws/practice_quote
pub async fn practice_quote(ws_upgrade: WebSocketUpgrade) -> impl IntoResponse {
    ws_upgrade.on_upgrade(async move |mut ws: WebSocket| -> Result<(), ()> {
        let mut auth: Auth;
        let mut identified: bool = false;

        let mut quote: String = "".to_string();
        // let mut quote_split: Vec<String>;

        let mut start: Instant;
        let mut started: bool = false;

        let mut index: usize = 0_usize;
        // let mut word_index: usize = 0_usize;

        let mut errors: usize = 0_usize;
        let mut typed: String = "".to_string();

        while let Some(message) = ws.recv().await {
            let message = if let Ok(message) = message {
                message.extract_json().map_err(|_| ())?
            } else {
                return Err(());
            };

            // TODO: disconnect rather than return from here
            match message {
                WebSocketInboundEvent::Identify { token, intent }
                    if matches!(intent, WebSocketIntent::PracticeQuote) =>
                {
                    auth = get_auth(token).await.map_err(|_| ())?;
                    identified = true;

                    let q = get_random_quote(auth).await.map_err(|_| ())?.json;

                    quote = (&q).content.clone();
                    // quote_split = quote.split(" ").into_iter().map(|o| o.to_string()).collect::<Vec<_>>();

                    WebSocketJsonResponse(
                        WebSocketOutboundEvent::PracticeQuoteReady {
                            quote: q,
                        }
                    )
                    .send(&mut ws)
                    .await
                    .map_err(|_| ())?;
                },
                WebSocketInboundEvent::Start if identified => {
                    start = Instant::now();
                    started = true;
                },
                WebSocketInboundEvent::KeyPress { key } if started => {
                    if key.eq("backspace") {
                        index -= if index > 0 && quote.chars().nth(index - 1).unwrap() == ' ' {
                            0
                        } else {
                            typed.pop();
                            1
                        };
                        continue;
                    }

                    // TODO: Ctrl+Backspace
                    // TODO: Word Impl

                    let current = quote.chars().nth(index).ok_or_else(|| ())?.to_string();

                    if key != current {
                        errors += 1;
                    }

                    typed.push_str(key.as_str());
                },
                _ => continue,
            };
        }

        Ok(())
    })
}
