use crate::types::Quote;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "op", content = "data")]
pub enum WebSocketInboundEvent {
    /// Sends information about who is connecting to the websocket and for what intent.
    /// This operation is required before any other events can be received.
    Identify {
        /// The session token which is to be used to identify who is connecting to the socket.
        token: String,

        /// What this connection will be used for.
        intent: WebSocketIntent,
    },

    /// Start the quote after the countdown and start timing.
    /// This signals the server to start receiving `KeyPress` events.
    Start,

    /// Gracefully quit this typing session.
    Quit,

    /// Signify that a key was pressed.
    KeyPress {
        /// The key which was pressed.
        key: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum WebSocketIntent {
    /// Typing practice with quotes.
    PracticeQuote = 0,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "op", content = "data")]
pub enum WebSocketOutboundEvent {
    /// Send quote data to the client.
    PracticeQuoteReady { quote: Quote },

    /// The user has finished typing the quote - give statistics.
    PracticeQuoteFinish {
        wpm: f32,
        wpm_raw: f32,
        accuracy: f32,
        replay: String,
        errors: u16,
    },
}
