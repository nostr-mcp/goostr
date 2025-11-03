use crate::storage;
use crate::util;
use anyhow::{Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct ProfileMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nip05: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lud06: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lud16: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct KeySettings {
    pub relays: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ProfileMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct SettingsFile {
    pub settings: BTreeMap<String, KeySettings>,
}

#[derive(Clone)]
pub struct SettingsStore {
    path: PathBuf,
    inner: Arc<RwLock<SettingsFile>>,
    pass: Arc<Vec<u8>>,
}

impl SettingsStore {
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

    pub async fn persist(&self) -> Result<()> {
        let data = { self.inner.read().await.clone() };
        storage::encrypt_to_file(&self.path, &self.pass, &data)
    }

    pub async fn get_settings(&self, pubkey_hex: &str) -> Option<KeySettings> {
        let data = self.inner.read().await;
        data.settings.get(pubkey_hex).cloned()
    }

    pub async fn save_settings(&self, pubkey_hex: String, settings: KeySettings) -> Result<()> {
        {
            let mut data = self.inner.write().await;
            data.settings.insert(pubkey_hex, settings);
        }
        self.persist().await
    }

    pub async fn remove_settings(&self, pubkey_hex: &str) -> Result<()> {
        {
            let mut data = self.inner.write().await;
            data.settings.remove(pubkey_hex);
        }
        self.persist().await
    }

    pub async fn all_settings(&self) -> BTreeMap<String, KeySettings> {
        let data = self.inner.read().await;
        data.settings.clone()
    }
}
