#![feature(async_closure)]
#![feature(once_cell)]
#![deny(unsafe_code)]

mod entrypoint;

#[tokio::main]
async fn main() {
    entrypoint::entrypoint().await;
}
