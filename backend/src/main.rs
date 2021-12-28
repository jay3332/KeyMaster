#![feature(async_closure)]
#![feature(never_type)]
#![feature(once_cell)]
#![feature(proc_macro_hygiene)]
#![deny(unsafe_code)]

pub mod auth;
pub mod database;
pub mod entrypoint;
#[macro_use]
pub mod macros;
pub mod json;
pub mod routes;
pub mod types;

extern crate dotenv;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    auth::start_rng();
    auth::initiate_hasher().await;
    database::start().await;
    entrypoint::entrypoint().await;
}
