use anyhow::{Context, Result};
use nostr_mcp_core::settings::SettingsStore as CoreSettingsStore;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone)]
pub struct SettingsStoreWrapper {
    inner: CoreSettingsStore,
}

impl SettingsStoreWrapper {
    pub async fn load_or_init(path: PathBuf) -> Result<Self> {
        let pass = crate::util::ensure_keystore_secret()?;
        let store = CoreSettingsStore::load_or_init(path, Arc::new(pass))
            .await
            .map_err(|e| anyhow::anyhow!(e))
            .context("decrypt settings file")?;
        Ok(Self { inner: store })
    }

    pub async fn persist(&self) -> Result<()> {
        self.inner.persist().await.map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn get_settings(&self, pubkey_hex: &str) -> Option<KeySettings> {
        self.inner.get_settings(pubkey_hex).await
    }

    pub async fn save_settings(&self, pubkey_hex: String, settings: KeySettings) -> Result<()> {
        self.inner
            .save_settings(pubkey_hex, settings)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn remove_settings(&self, pubkey_hex: &str) -> Result<()> {
        self.inner
            .remove_settings(pubkey_hex)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn all_settings(&self) -> BTreeMap<String, KeySettings> {
        self.inner.all_settings().await
    }
}

pub use nostr_mcp_core::settings::{FollowEntry, KeySettings, ProfileMetadata};

pub type SettingsStore = SettingsStoreWrapper;
