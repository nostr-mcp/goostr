use anyhow::{Context, Result};
use nostr_mcp_core::settings::SettingsStore as CoreSettingsStore;
use std::path::PathBuf;
use std::sync::Arc;

pub use nostr_mcp_core::settings::{FollowEntry, KeySettings, ProfileMetadata};

pub type SettingsStore = CoreSettingsStore;

pub async fn load_or_init(path: PathBuf) -> Result<SettingsStore> {
    let pass = crate::util::ensure_keystore_secret()?;
    CoreSettingsStore::load_or_init(path, Arc::new(pass))
        .await
        .map_err(|e| anyhow::anyhow!(e))
        .context("decrypt settings file")
}
