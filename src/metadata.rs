use anyhow::Result;

pub use nostr_mcp_core::metadata::{args_to_profile, FetchMetadataArgs, MetadataResult, SetMetadataArgs};
use nostr_mcp_core::metadata as core_metadata;
pub use nostr_mcp_core::settings::ProfileMetadata;
use nostr_sdk::prelude::*;

pub async fn publish_metadata(client: &Client, profile: &ProfileMetadata) -> Result<MetadataResult> {
    core_metadata::publish_metadata(client, profile)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn fetch_metadata(client: &Client, pubkey: &PublicKey) -> Result<Option<Metadata>> {
    core_metadata::fetch_metadata(client, pubkey)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}
