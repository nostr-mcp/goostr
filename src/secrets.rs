use anyhow::Result;
use nostr_mcp_core::secrets::{KeyringSecretStore, SecretStore};
use std::sync::Arc;

const SERVICE: &str = "goostr";

fn keyring_store() -> KeyringSecretStore {
    KeyringSecretStore::new(SERVICE)
}

pub fn secret_store() -> Arc<dyn SecretStore> {
    Arc::new(keyring_store())
}

pub fn set(label: &str, secret: &str) -> Result<()> {
    keyring_store().set(label, secret).map_err(|e| anyhow::anyhow!(e))
}

pub fn get(label: &str) -> Result<Option<String>> {
    keyring_store().get(label).map_err(|e| anyhow::anyhow!(e))
}

pub fn delete(label: &str) -> Result<()> {
    keyring_store().delete(label).map_err(|e| anyhow::anyhow!(e))
}
