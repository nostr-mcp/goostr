use anyhow::{anyhow, Result};
use std::path::Path;

pub fn encrypt_to_file<T: serde::Serialize>(
    path: &Path,
    pass: &[u8],
    value: &T,
) -> Result<()> {
    nostr_mcp_core::storage::encrypt_to_file(path, pass, value)
        .map_err(|e| anyhow!(e))
}

pub fn decrypt_from_file<T: serde::de::DeserializeOwned>(
    path: &Path,
    pass: &[u8],
) -> Result<T> {
    nostr_mcp_core::storage::decrypt_from_file(path, pass).map_err(|e| anyhow!(e))
}
