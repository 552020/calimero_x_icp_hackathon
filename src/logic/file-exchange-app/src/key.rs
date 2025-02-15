use ed25519_dalek::{Signature, SigningKey, VerifyingKey};
use crate::repr::ReprBytes;

impl ReprBytes for VerifyingKey {
    type Bytes = [u8; 32];

    fn to_bytes(&self) -> Self::Bytes {
        self.to_bytes()
    }

    fn from_bytes<F, E>(f: F) -> Option<Result<Self, E>>
    where
        F: FnOnce(&mut Self::Bytes) -> Option<E>,
    {
        let mut bytes = [0; 32];
        if let Some(err) = f(&mut bytes) {
            return Some(Err(err));
        }
        Some(Ok(VerifyingKey::from_bytes(&bytes).ok()?))
    }
}

impl ReprBytes for SigningKey {
    type Bytes = [u8; 32];

    fn to_bytes(&self) -> Self::Bytes {
        self.to_bytes()
    }

    fn from_bytes<F, E>(f: F) -> Option<Result<SigningKey, E>>
    where
        F: FnOnce(&mut Self::Bytes) -> Option<E>,
    {
        let mut bytes = [0; 32];
        Some(f(&mut bytes).map_or_else(|| Ok(SigningKey::from_bytes(&bytes)), Err))
    }
}

impl ReprBytes for Signature {
    type Bytes = [u8; 64];

    fn to_bytes(&self) -> Self::Bytes {
        self.to_bytes()
    }

    fn from_bytes<F, E>(f: F) -> Option<Result<Signature, E>>
    where
        F: FnOnce(&mut Self::Bytes) -> Option<E>,
    {
        let mut bytes = [0; 64];
        Some(f(&mut bytes).map_or_else(|| Ok(Signature::from_bytes(&bytes)), Err))
    }
}
