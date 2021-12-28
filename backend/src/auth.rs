use argon2_async::{set_config, Config};

use base64::{decode, encode};
use ring::rand::{SecureRandom, SystemRandom};

use std::lazy::SyncOnceCell;
use std::sync::atomic::{AtomicU16, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static COUNTER: AtomicU16 = AtomicU16::new(0);

pub static RNG: SyncOnceCell<SystemRandom> = SyncOnceCell::new();
pub const EPOCH: u128 = 1_577_836_800_000; // Jan 1 2020 @ 00:00:00 UTC

pub async fn initiate_hasher() {
    set_config(Config::new_insecure()).await;
}

pub fn start_rng() {
    RNG.set(SystemRandom::new()).expect("Failed to start RNG.");
}

/// Generates a token from a snowflake ID.
/// {id to string to b64}.{timestamp to string to b64}.{32 random bytes to b64}
pub fn generate_token(id: u64) -> String {
    let first = {
        let stringified = id.to_string();
        encode(stringified.as_bytes())
    };

    let second = {
        let epoch = get_epoch_time();
        encode(epoch.to_string().as_bytes())
    };

    let third = {
        let mut v = vec![0; 32];
        RNG.get().expect("Failed to get RNG").fill(&mut v).ok();
        encode(v.as_slice())
    };

    format!("{}.{}.{}", first, second, third)
}

pub fn decode_token(token: String) -> Option<u64> {
    let first = token.split(".").next()?;

    match String::from_utf8(decode(first).ok()?) {
        Ok(o) => u64::from_str_radix(&o, 10).ok(),
        Err(_) => None,
    }
}

pub fn get_epoch_time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System clock is currently behind Unix Epoch.")
        .as_millis()
        .saturating_sub(EPOCH)
}

/// Generates a snowflake.
///
/// Snowflakes are 64 bit unsigned integers:
/// The first 48 bits contain the amount of seconds since the KeyMaster epoch.
/// The last 16 bits contain the increment number of an internal counter, allowing up to 4096 unique snowflakes per millisecond.
pub fn generate_snowflake() -> u64 {
    ((get_epoch_time() as u64) << 16) + (COUNTER.fetch_add(1, Ordering::Relaxed) as u64)
}
