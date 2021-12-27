use axum::body::{self, BoxBody, Bytes};
use axum::response::IntoResponse;
use http::header::CONTENT_TYPE;
use http::{HeaderValue, Response};
use serde::Serialize;

pub struct JsonResponse<T: Serialize> {
    /// The HTTP status code of this response.
    pub status: u16,
    
    /// The JSON data represented as a Rust object.
    pub json: T,
}

impl<T> JsonResponse<T> where T: Serialize {
    pub fn new(status: u16, json: T) -> Self {
        Self {
            status,
            json,
        }
    }
}

fn build_response_from(status: u16, content_type: &'static str, data: body::Full<Bytes>) -> Response<BoxBody> {
    Response::builder()
        .status(status)
        .header(CONTENT_TYPE, HeaderValue::from_static(content_type))
        .body(body::boxed(data))
        .expect("Could not create a response.")
}

impl<T> IntoResponse for JsonResponse<T> where T: Serialize {
    fn into_response(self) -> Response<BoxBody> {
        let data = match simd_json::to_vec(&self.json) {
            Ok(data) => data,
            Err(err) => return build_response_from(
                500,
                "text/plain",
                body::Full::from(err.to_string()),
            ),
        };

        build_response_from(
            self.status,
            "application/json",
            body::Full::from(data),
        )
    }
}