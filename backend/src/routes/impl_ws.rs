use crate::types::WebSocketInboundEvent;
use axum::extract::ws::Message;

pub trait ExtractJson {
    fn extract_json(&self) -> Result<WebSocketInboundEvent, ()>;
}

impl ExtractJson for Message {
    fn extract_json(&self) -> Result<WebSocketInboundEvent, ()> {
        match self {
            Message::Text(text) => Ok(simd_json::from_str(&mut text.clone()).map_err(|_| ())?),
            Message::Binary(bin) => {
                Ok(simd_json::from_slice(&mut bin.to_owned()[..]).map_err(|_| ())?)
            }
            _ => Err(()),
        }
    }
}
