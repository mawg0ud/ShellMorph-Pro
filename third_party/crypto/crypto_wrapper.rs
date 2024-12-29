// crypto_wrapper.rs - Abstraction over external cryptographic library

use sodiumoxide::crypto::secretbox;
use sodiumoxide::init as sodium_init;

pub struct CryptoWrapper;

impl CryptoWrapper {
    /// Initializes the cryptographic library.
    pub fn initialize() -> bool {
        sodium_init().is_ok()
    }

    /// Encrypts data using libsodium's secretbox.
    pub fn encrypt(data: &[u8], key: &[u8; secretbox::KEYBYTES]) -> Vec<u8> {
        let nonce = secretbox::gen_nonce();
        let ciphertext = secretbox::seal(data, &nonce, &secretbox::Key::from_slice(key).unwrap());

        [&nonce.0[..], &ciphertext].concat()
    }

    /// Decrypts data using libsodium's secretbox.
    pub fn decrypt(encrypted_data: &[u8], key: &[u8; secretbox::KEYBYTES]) -> Option<Vec<u8>> {
        let nonce = secretbox::Nonce::from_slice(&encrypted_data[..secretbox::NONCEBYTES])?;
        let ciphertext = &encrypted_data[secretbox::NONCEBYTES..];
        secretbox::open(ciphertext, &nonce, &secretbox::Key::from_slice(key).unwrap()).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::CryptoWrapper;

    #[test]
    fn test_encrypt_decrypt() {
        assert!(CryptoWrapper::initialize());

        let key = [0xAA; 32];
        let data = b"Sensitive data";

        let encrypted = CryptoWrapper::encrypt(data, &key);
        let decrypted = CryptoWrapper::decrypt(&encrypted, &key).unwrap();

        assert_eq!(decrypted, data);
    }
}
