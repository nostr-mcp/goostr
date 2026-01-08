use anyhow::Result;

pub use nostr_mcp_core::follows::{
    AddFollowArgs, FollowsResult, PublishFollowsResult, RemoveFollowArgs, SetFollowsArgs,
};
use nostr_mcp_core::follows as core_follows;
pub use nostr_mcp_core::settings::FollowEntry;
use nostr_sdk::prelude::*;

pub async fn fetch_follows(client: &Client, pubkey: &PublicKey) -> Result<Vec<FollowEntry>> {
    core_follows::fetch_follows(client, pubkey)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn publish_follows(
    client: &Client,
    follows: &[FollowEntry],
) -> Result<PublishFollowsResult> {
    core_follows::publish_follows(client, follows)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn sync_follows(
    client: &Client,
    pubkey: &PublicKey,
    local_follows: Vec<FollowEntry>,
) -> Result<(Vec<FollowEntry>, bool)> {
    core_follows::sync_follows(client, pubkey, local_follows)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}
