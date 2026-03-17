use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use super::crypto::{encrypt, decrypt};

pub struct EnvManager {
    nexus_dir: PathBuf,
}

impl EnvManager {
    pub fn new(nexus_dir: &Path) -> Self {
        Self {
            nexus_dir: nexus_dir.to_path_buf(),
        }
    }

    pub fn add_environment(&self, name: &str, content: &str) -> Result<()> {
        let env_path = self.nexus_dir.join(format!("{}.env", name));
        fs::write(&env_path, content)
            .context("Failed to write environment file")?;
        Ok(())
    }

    pub fn has_environment(&self, name: &str) -> bool {
        self.nexus_dir.join(format!("{}.env", name)).exists()
    }

    pub fn has_encrypted(&self, name: &str) -> bool {
        self.nexus_dir.join(format!("{}.enc", name)).exists()
    }

    pub fn encrypt_environment(&self, name: &str, password: &str) -> Result<()> {
        let env_path = self.nexus_dir.join(format!("{}.env", name));
        let enc_path = self.nexus_dir.join(format!("{}.enc", name));

        let content = fs::read_to_string(&env_path)
            .context("Failed to read environment file")?;

        let encrypted = encrypt(&content, password)
            .context("Encryption failed")?;

        fs::write(&enc_path, encrypted)
            .context("Failed to write encrypted file")?;

        // Remove plaintext file
        fs::remove_file(&env_path)
            .context("Failed to remove plaintext file")?;

        Ok(())
    }

    pub fn decrypt_environment(&self, name: &str, password: &str) -> Result<()> {
        let env_path = self.nexus_dir.join(format!("{}.env", name));
        let enc_path = self.nexus_dir.join(format!("{}.enc", name));

        let encrypted = fs::read_to_string(&enc_path)
            .context("Failed to read encrypted file")?;

        let decrypted = decrypt(&encrypted, password)
            .context("Decryption failed")?;

        fs::write(&env_path, decrypted)
            .context("Failed to write decrypted file")?;

        // Remove encrypted file
        fs::remove_file(&enc_path)
            .context("Failed to remove encrypted file")?;

        Ok(())
    }

    pub fn get_environment(&self, name: &str) -> Result<String> {
        let env_path = self.nexus_dir.join(format!("{}.env", name));
        
        if !env_path.exists() {
            return Err(anyhow::anyhow!("Environment {} not found", name));
        }

        fs::read_to_string(&env_path)
            .context("Failed to read environment file")
    }

    pub fn generate_example(&self, original_path: &Path) -> Result<()> {
        let content = fs::read_to_string(original_path)
            .context("Failed to read original .env file")?;

        let example_content = self.strip_values(&content);

        let example_path = PathBuf::from(".env.example");
        fs::write(&example_path, example_content)
            .context("Failed to write .env.example")?;

        Ok(())
    }

    fn strip_values(&self, content: &str) -> String {
        content
            .lines()
            .map(|line| {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    return line.to_string();
                }
                
                if let Some((key, _)) = line.split_once('=') {
                    format!("{}=", key.trim())
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
