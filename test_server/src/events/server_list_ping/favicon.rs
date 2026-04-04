use base64::{Engine as _, engine::general_purpose};
use std::fs;

pub fn png_to_base64(path: &str) -> Option<String> {
    if let Ok(bytes) = fs::read(path) {
        let b64 = general_purpose::STANDARD.encode(bytes);
        return Some(format!("data:image/png;base64,{}", b64));
    }
    None
}
