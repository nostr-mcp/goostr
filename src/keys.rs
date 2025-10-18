use crate::secrets;
use crate::storage;
use crate::util;
use anyhow::{anyhow, Context, Result};
use nostr::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use zeroize::Zeroize;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct KeyEntry {
    pub label: String,
    pub public_key: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct KeyFile {
    pub active: Option<String>,
    pub keys: BTreeMap<String, KeyEntry>,
}

#[derive(Clone)]
pub struct KeyStore {
    path: PathBuf,
    inner: Arc<RwLock<KeyFile>>,
    pass: Arc<Vec<u8>>,
}

impl KeyStore {
    pub async fn load_or_init(path: PathBuf) -> Result<Self> {
        util::ensure_parent_dir(&path)?;
        let pass = util::ensure_keystore_secret()?;
        let pass_arc = Arc::new(pass);
        let data = if path.exists() {
            storage::decrypt_from_file::<KeyFile>(&path, &pass_arc).context("decrypt index")?
        } else if util::legacy_keys_json_path().exists() {
            let s = fs::read_to_string(util::legacy_keys_json_path())?;
            let legacy: KeyFile = serde_json::from_str(&s)?;
            storage::encrypt_to_file(&path, &pass_arc, &legacy)?;
            let _ = fs::remove_file(util::legacy_keys_json_path());
            legacy
        } else {
            KeyFile::default()
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

    pub async fn list(&self) -> Vec<KeyEntry> {
        let data = self.inner.read().await;
        data.keys.values().cloned().collect()
    }

    pub async fn get_active(&self) -> Option<KeyEntry> {
        let data = self.inner.read().await;
        match &data.active {
            Some(label) => data.keys.get(label).cloned(),
            None => None,
        }
    }

    pub async fn set_active(&self, label: String) -> Result<KeyEntry> {
        let mut data = self.inner.write().await;
        if !data.keys.contains_key(&label) {
            return Err(anyhow!("unknown key label"));
        }
        data.active = Some(label.clone());
        let entry = data.keys.get(&label).cloned().unwrap();
        drop(data);
        self.persist().await?;
        Ok(entry)
    }

    pub async fn remove(&self, label: String) -> Result<Option<KeyEntry>> {
        let mut data = self.inner.write().await;
        let removed = data.keys.remove(&label);
        if data.active.as_deref() == Some(&label) {
            data.active = None;
        }
        drop(data);
        self.persist().await?;
        let _ = secrets::delete(&label);
        Ok(removed)
    }

    pub async fn import_secret(
        &self,
        label: String,
        secret: String,
        make_active: bool,
        persist_secret: bool,
    ) -> Result<KeyEntry> {
        let keys = if secret.starts_with("nsec1") || secret.starts_with("npub1") {
            Keys::parse(&secret)?
        } else {
            return Err(anyhow!("unsupported key material"));
        };
        self.insert_keys(label, keys, make_active, persist_secret)
            .await
    }

    pub async fn generate(
        &self,
        label: String,
        make_active: bool,
        persist_secret: bool,
    ) -> Result<KeyEntry> {
        let keys = Keys::generate();
        self.insert_keys(label, keys, make_active, persist_secret)
            .await
    }

    async fn insert_keys(
        &self,
        label: String,
        keys: Keys,
        make_active: bool,
        persist_secret: bool,
    ) -> Result<KeyEntry> {
        let public_key = keys.public_key().to_bech32()?;
        if persist_secret {
            let mut sk = keys.secret_key().to_bech32()?;
            secrets::set(&label, &sk)?;
            sk.zeroize();
        }
        let entry = KeyEntry {
            label: label.clone(),
            public_key,
            created_at: chrono::Utc::now().timestamp(),
        };
        let mut data = self.inner.write().await;
        data.keys.insert(label.clone(), entry.clone());
        if make_active {
            data.active = Some(label);
        }
        drop(data);
        self.persist().await?;
        Ok(entry)
    }

    pub async fn rename_label(&self, from: String, to: String) -> Result<KeyEntry> {
        if from == to {
            return Err(anyhow!("new label must be different"));
        }
        let mut data = self.inner.write().await;
        if !data.keys.contains_key(&from) {
            return Err(anyhow!("unknown key label"));
        }
        if data.keys.contains_key(&to) {
            return Err(anyhow!("label already exists"));
        }
        let mut entry = data
            .keys
            .remove(&from)
            .ok_or_else(|| anyhow!("unknown key label"))?;
        entry.label = to.clone();
        data.keys.insert(to.clone(), entry.clone());
        if data.active.as_deref() == Some(&from) {
            data.active = Some(to.clone());
        }
        drop(data);
        self.persist().await?;
        if let Some(sk) = secrets::get(&from)? {
            let _ = secrets::delete(&from);
            secrets::set(&to, &sk)?;
        }
        Ok(entry)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GenerateArgs {
    pub label: String,
    pub make_active: Option<bool>,
    pub persist_secret: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ImportArgs {
    pub label: String,
    pub key_material: String,
    pub make_active: Option<bool>,
    pub persist_secret: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RemoveArgs {
    pub label: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetActiveArgs {
    pub label: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RenameLabelArgs {
    pub from: Option<String>,
    pub to: String,
}

#[derive(Debug, Deserialize, JsonSchema, Default)]
pub struct EmptyArgs {}
