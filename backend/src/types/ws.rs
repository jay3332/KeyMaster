use crate::types::Quote;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "op", content = "data")]
pub enum WebSocketInboundEvent {
    Identify {
        /// The session token which is to be used to identify who is connecting to the socket.
        token: String,

        /// What this connection will be used for.
        intent: WebSocketIntent,
    },

    KeyPress {
        /// The key which was pressed.
        key: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum WebSocketIntent {
    /// Typing practice with quotes.
    PracticeQuote = 0,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "op", content = "data")]
pub enum WebSocketOutboundEvent {
    /// Send quote data to the client.
    PracticeQuoteReady { quote: Quote },

    /// The user has finished typing the quote - give statistics.
    PracticeQuoteFinish {
        quote: Quote,
        wpm: f32,
        wpm_raw: f32,
        accuracy: f32,
        replay: String,
    },
}
