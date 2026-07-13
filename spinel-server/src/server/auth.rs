use rsa::RsaPrivateKey;
use rsa::pkcs8::EncodePublicKey;
use rsa::rand_core::OsRng;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Auth {
    Offline,
    Online(OnlineAuth),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OnlineAuth {
    private_key: RsaPrivateKey,
    public_key_der: Vec<u8>,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateOnlineAuthError {
    #[error("failed to generate online authentication key pair: {reason}")]
    GenerateKeyPair { reason: String },

    #[error("failed to encode online authentication public key: {reason}")]
    EncodePublicKey { reason: String },
}

impl OnlineAuth {
    pub fn new() -> Result<Self, CreateOnlineAuthError> {
        let private_key = RsaPrivateKey::new(&mut OsRng, 1024).map_err(|error| {
            CreateOnlineAuthError::GenerateKeyPair {
                reason: error.to_string(),
            }
        })?;
        let public_key_der = private_key
            .to_public_key()
            .to_public_key_der()
            .map_err(|error| CreateOnlineAuthError::EncodePublicKey {
                reason: error.to_string(),
            })?;

        Ok(Self {
            private_key,
            public_key_der: public_key_der.as_ref().to_vec(),
        })
    }

    pub fn get_private_key(&self) -> &RsaPrivateKey {
        &self.private_key
    }

    pub fn get_public_key_der(&self) -> &[u8] {
        &self.public_key_der
    }
}
