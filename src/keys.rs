use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::sync::Arc;

pub use nostr_mcp_core::key_store::{
    EmptyArgs, ExportArgs, ExportFormat, ExportResult, GenerateArgs, ImportArgs, KeyEntry, KeyFile,
    KeyStore, RemoveArgs, RenameLabelArgs, SetActiveArgs,
};
pub use nostr_mcp_core::keys::{
    derive_public_from_private, verify_key, DerivePublicArgs, DerivePublicResult, KeyType,
    VerifyArgs, VerifyResult,
};

pub async fn load_or_init_keystore(path: PathBuf) -> Result<KeyStore> {
    let pass = crate::util::ensure_keystore_secret()?;
    let secrets = crate::secrets::secret_store();
    let legacy_path = Some(crate::util::legacy_keys_json_path());
    KeyStore::load_or_init(path, Arc::new(pass), secrets, legacy_path)
        .await
        .map_err(|e| anyhow!(e))
}
