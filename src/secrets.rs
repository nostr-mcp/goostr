use anyhow::Result;
use nostr_mcp_core::secrets::{KeyringSecretStore, SecretStore};

const SERVICE: &str = "goostr";

fn store() -> KeyringSecretStore {
    KeyringSecretStore::new(SERVICE)
}

pub fn set(label: &str, secret: &str) -> Result<()> {
    store().set(label, secret).map_err(|e| anyhow::anyhow!(e))
}

pub fn get(label: &str) -> Result<Option<String>> {
    store().get(label).map_err(|e| anyhow::anyhow!(e))
}

pub fn delete(label: &str) -> Result<()> {
    store().delete(label).map_err(|e| anyhow::anyhow!(e))
}
