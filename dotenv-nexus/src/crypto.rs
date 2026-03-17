use anyhow::{Context, Result};
use aes_gcm::{Aes256Gcm, Key, Nonce, KeyInit};
use aes_gcm::aead::Aead;
use base64::{Engine as _, engine::general_purpose};
use rand::Rng;
use sha2::{Sha256, Digest};
use generic_array::GenericArray;

pub fn encrypt(plaintext: &str, password: &str) -> Result<String> {
    let key = derive_key(password);
    let cipher = Aes256Gcm::new(&key);

    let mut rng = rand::thread_rng();
    let mut nonce_bytes = [0u8; 12];
    rng.fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let plaintext_bytes = plaintext.as_bytes();
    let ciphertext = cipher
        .encrypt(nonce, plaintext_bytes)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

    let mut result = nonce_bytes.to_vec();
    result.extend(ciphertext);

    Ok(general_purpose::STANDARD.encode(&result))
}

pub fn decrypt(ciphertext: &str, password: &str) -> Result<String> {
    let key = derive_key(password);
    let cipher = Aes256Gcm::new(&key);

    let decoded = general_purpose::STANDARD
        .decode(ciphertext)
        .context("Invalid base64")?;

    if decoded.len() < 12 {
        return Err(anyhow::anyhow!("Invalid ciphertext format"));
    }

    let (nonce_bytes, ciphertext_bytes) = decoded.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext_bytes)
        .map_err(|e| anyhow::anyhow!("Decryption failed (wrong password?): {}", e))?;

    String::from_utf8(plaintext).context("Invalid UTF-8 in decrypted data")
}

fn derive_key(password: &str) -> Key<Aes256Gcm> {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(b".env-nexus-salt");
    let hash = hasher.finalize();
    
    // Use first 32 bytes as AES-256 key
    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&hash[..32]);
    
    GenericArray::clone_from_slice(&key_bytes)
}
