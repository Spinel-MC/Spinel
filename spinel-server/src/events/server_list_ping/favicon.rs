use base64::{Engine as _, engine::general_purpose};

pub struct Favicon {
    pub base64: String
}

impl Favicon {
    pub fn from_bytes(data: impl AsRef<[u8]>) -> Self {
        let b64 = general_purpose::STANDARD.encode(data.as_ref());
        Self {
            base64: format!("data:image/png;base64,{}", b64),
        }
    }

    pub fn from_base64(data: String) -> Result<Self, base64::DecodeError> {
            general_purpose::STANDARD.decode(&data)?;

        Ok(Self {
            base64: format!("data:image/png;base64,{}", data),
        })
    }
}