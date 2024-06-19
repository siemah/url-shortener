use base64;
use base64::Engine;
use crate::config::Config;

pub fn convert_to_id(number: i64) -> String {
    let bytes = number.to_be_bytes(); // Convert the number to bytes
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&bytes) // Convert the bytes to a base64 string
}