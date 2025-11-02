use crate::storage;
use crate::util;
use anyhow::{Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Settings for a specific Nostr key
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct KeySettings {
    /// List of relay URLs configured for this key
    pub relays: Vec<String>,
}

/// File format for persisting all key settings
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct SettingsFile {
    /// Map from public key (hex32) to settings
    pub settings: BTreeMap<String, KeySettings>,
}

/// Store for managing encrypted key-specific settings
#[derive(Clone)]
pub struct SettingsStore {
    path: PathBuf,
    inner: Arc<RwLock<SettingsFile>>,
    pass: Arc<Vec<u8>>,
}

impl SettingsStore {
    /// Load settings from encrypted file or initialize empty
    pub async fn load_or_init(path: PathBuf) -> Result<Self> {
        util::ensure_parent_dir(&path)?;
        let pass = util::ensure_keystore_secret()?;
        let pass_arc = Arc::new(pass);
        
        let data = if path.exists() {
            storage::decrypt_from_file::<SettingsFile>(&path, &pass_arc)
                .context("decrypt settings file")?
        } else {
            SettingsFile::default()
        };
        
        Ok(Self {
            path,
            inner: Arc::new(RwLock::new(data)),
            pass: pass_arc,
        })
    }

    /// Persist settings to encrypted file
    pub async fn persist(&self) -> Result<()> {
        let data = { self.inner.read().await.clone() };
        storage::encrypt_to_file(&self.path, &self.pass, &data)
    }

    /// Get settings for a specific public key (hex format)
    pub async fn get_settings(&self, pubkey_hex: &str) -> Option<KeySettings> {
        let data = self.inner.read().await;
        data.settings.get(pubkey_hex).cloned()
    }

    /// Save settings for a specific public key (hex format)
    pub async fn save_settings(&self, pubkey_hex: String, settings: KeySettings) -> Result<()> {
        {
            let mut data = self.inner.write().await;
            data.settings.insert(pubkey_hex, settings);
        }
        self.persist().await
    }

    /// Remove settings for a specific public key
    pub async fn remove_settings(&self, pubkey_hex: &str) -> Result<()> {
        {
            let mut data = self.inner.write().await;
            data.settings.remove(pubkey_hex);
        }
        self.persist().await
    }

    /// Get all settings (for debugging)
    pub async fn all_settings(&self) -> BTreeMap<String, KeySettings> {
        let data = self.inner.read().await;
        data.settings.clone()
    }
}
