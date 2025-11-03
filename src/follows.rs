use crate::settings::FollowEntry;
use anyhow::Result;
use nostr_sdk::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetFollowsArgs {
    pub follows: Vec<FollowEntry>,
    pub publish: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddFollowArgs {
    pub pubkey: String,
    pub relay_url: Option<String>,
    pub petname: Option<String>,
    pub publish: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RemoveFollowArgs {
    pub pubkey: String,
    pub publish: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct FollowsResult {
    pub follows: Vec<FollowEntry>,
    pub count: usize,
}

#[derive(Debug, Serialize)]
pub struct PublishFollowsResult {
    pub saved: bool,
    pub published: bool,
    pub event_id: Option<String>,
    pub pubkey: Option<String>,
    pub success_relays: Vec<String>,
    pub failed_relays: HashMap<String, String>,
}

pub async fn fetch_follows(client: &Client, pubkey: &PublicKey) -> Result<Vec<FollowEntry>> {
    let filter = Filter::new()
        .author(*pubkey)
        .kind(Kind::ContactList)
        .limit(1);

    let events = client
        .fetch_events(filter, std::time::Duration::from_secs(10))
        .await?;

    if let Some(event) = events.into_iter().next() {
        let mut follows = Vec::new();
        for tag in event.tags.iter() {
            if tag.kind() == TagKind::p() {
                if let Some(pubkey_str) = tag.content() {
                    let pubkey = pubkey_str.to_string();
                    let tag_vec = tag.clone().to_vec();
                    let relay_url = tag_vec.get(2).cloned().filter(|s| !s.is_empty());
                    let petname = tag_vec.get(3).cloned().filter(|s| !s.is_empty());
                    follows.push(FollowEntry {
                        pubkey,
                        relay_url,
                        petname,
                    });
                }
            }
        }
        Ok(follows)
    } else {
        Ok(Vec::new())
    }
}

pub async fn publish_follows(
    client: &Client,
    follows: &[FollowEntry],
) -> Result<PublishFollowsResult> {
    let mut tags = Vec::new();

    for follow in follows {
        let mut tag_values = vec!["p".to_string(), follow.pubkey.clone()];
        if let Some(ref relay) = follow.relay_url {
            tag_values.push(relay.clone());
        } else {
            tag_values.push(String::new());
        }
        if let Some(ref petname) = follow.petname {
            tag_values.push(petname.clone());
        }

        tags.push(Tag::parse(&tag_values)?);
    }

    let builder = EventBuilder::new(Kind::ContactList, "").tags(tags);

    let out = client.send_event_builder(builder).await?;
    let signer_pubkey = client.signer().await?.get_public_key().await?.to_hex();

    let event_id = out.id().to_string();
    let success = out.success.into_iter().map(|u| u.to_string()).collect();
    let failed = out
        .failed
        .into_iter()
        .map(|(u, e)| (u.to_string(), e.to_string()))
        .collect();

    Ok(PublishFollowsResult {
        saved: true,
        published: true,
        event_id: Some(event_id),
        pubkey: Some(signer_pubkey),
        success_relays: success,
        failed_relays: failed,
    })
}

pub async fn sync_follows(
    client: &Client,
    pubkey: &PublicKey,
    local_follows: Vec<FollowEntry>,
) -> Result<(Vec<FollowEntry>, bool)> {
    let relay_follows = fetch_follows(client, pubkey).await?;

    if local_follows != relay_follows {
        if !local_follows.is_empty() {
            publish_follows(client, &local_follows).await?;
            Ok((local_follows, true))
        } else {
            Ok((relay_follows, false))
        }
    } else {
        Ok((local_follows, false))
    }
}
