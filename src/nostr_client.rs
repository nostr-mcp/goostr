use crate::error::GoostrError;
use crate::keys::KeyStore;
use crate::settings::SettingsStore;
use nostr_mcp_core::client as core_client;
use std::sync::Arc;

pub use nostr_mcp_core::client::ActiveClient;

pub async fn ensure_client(
    ks: Arc<KeyStore>,
    settings_store: Arc<SettingsStore>,
) -> Result<ActiveClient, GoostrError> {
    core_client::ensure_client(ks, settings_store)
        .await
        .map_err(|e| GoostrError::invalid(e.to_string()))
}

pub async fn reset_cached_client() -> Result<(), GoostrError> {
    core_client::reset_cached_client()
        .await
        .map_err(|e| GoostrError::invalid(e.to_string()))
}
