use anyhow::Result;
use nostr_sdk::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RelaysSetArgs {
    pub urls: Vec<String>,
    pub read_write: Option<String>,
    pub autoconnect: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RelaysConnectArgs {
    pub urls: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RelaysDisconnectArgs {
    pub urls: Option<Vec<String>>,
    pub force_remove: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct RelayStatusRow {
    pub url: String,
    pub status: String,
    pub read: bool,
    pub write: bool,
    pub discovery: bool,
}

#[derive(Debug, Serialize)]
pub struct SendResult {
    pub id: String,
    pub success: Vec<String>,
    pub failed: HashMap<String, String>,
    pub pubkey: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PostTextArgs {
    pub content: String,
    pub pow: Option<u8>,
    pub to_relays: Option<Vec<String>>,
}

pub async fn set_relays(client: &Client, args: RelaysSetArgs) -> Result<()> {
    let rw = args
        .read_write
        .as_deref()
        .unwrap_or("both")
        .to_ascii_lowercase();
    for u in args.urls {
        match rw.as_str() {
            "read" => client.add_read_relay(&u).await?,
            "write" => client.add_write_relay(&u).await?,
            "both" => client.add_relay(&u).await?,
            _ => {
                return Err(anyhow::anyhow!(
                    "read_write must be one of: read, write, both"
                ))
            }
        };
    }
    if args.autoconnect.unwrap_or(true) {
        client.connect().await;
    }
    Ok(())
}

pub async fn connect_relays(client: &Client, args: RelaysConnectArgs) -> Result<()> {
    if let Some(urls) = args.urls {
        for u in urls {
            client.connect_relay(&u).await?;
        }
    } else {
        client.connect().await;
    }
    Ok(())
}

pub async fn disconnect_relays(client: &Client, args: RelaysDisconnectArgs) -> Result<()> {
    let force = args.force_remove.unwrap_or(false);
    if let Some(urls) = args.urls {
        for u in urls {
            if force {
                client.force_remove_relay(&u).await?;
            } else {
                client.remove_relay(&u).await?;
            }
        }
    } else if force {
        client.force_remove_all_relays().await;
    } else {
        client.remove_all_relays().await;
    }
    Ok(())
}

pub async fn list_relays(client: &Client) -> Result<Vec<RelayStatusRow>> {
    let map = client.relays().await;
    let mut out = Vec::with_capacity(map.len());
    for (url, r) in map {
        let flags = r.flags();
        let row = RelayStatusRow {
            url: url.to_string(),
            status: r.status().to_string(),
            read: flags.has(RelayServiceFlags::READ, FlagCheck::Any),
            write: flags.has(RelayServiceFlags::WRITE, FlagCheck::Any),
            discovery: flags.has(RelayServiceFlags::DISCOVERY, FlagCheck::Any),
        };
        out.push(row);
    }
    Ok(out)
}

pub async fn get_relay_urls(client: &Client) -> Vec<String> {
    let map = client.relays().await;
    map.keys().map(|url| url.to_string()).collect()
}

pub async fn subscription_targets_my_notes(
    pk: PublicKey,
    since: Option<Timestamp>,
    until: Option<Timestamp>,
) -> Filter {
    let default_since = Timestamp::now() - 86400 * 7;
    let mut filter = Filter::new()
        .author(pk)
        .kind(Kind::TextNote)
        .since(since.unwrap_or(default_since));

    if let Some(u) = until {
        filter = filter.until(u);
    }
    filter
}

pub async fn subscription_targets_mentions_me(
    pk: PublicKey,
    since: Option<Timestamp>,
    until: Option<Timestamp>,
) -> Filter {
    let default_since = Timestamp::now() - 86400 * 7;
    let needle = pk.to_string();
    let mut filter = Filter::new()
        .kind(Kind::TextNote)
        .search(needle)
        .since(since.unwrap_or(default_since));

    if let Some(u) = until {
        filter = filter.until(u);
    }
    filter
}

pub async fn subscription_targets_my_metadata(pk: PublicKey) -> Filter {
    Filter::new().author(pk).kind(Kind::Metadata).limit(1)
}

pub async fn list_events(client: &Client, filter: Filter, timeout_secs: u64) -> Result<Vec<Event>> {
    let events = client
        .fetch_events(filter, std::time::Duration::from_secs(timeout_secs))
        .await?;
    Ok(events.into_iter().collect())
}

pub async fn status_summary(client: &Client) -> Result<HashMap<String, String>> {
    let mut m = HashMap::new();
    let connected = client.relays().await;
    m.insert("relay_count".into(), connected.len().to_string());
    Ok(m)
}

pub async fn publish_event_builder(
    client: &Client,
    builder: EventBuilder,
    to_relays: Option<Vec<String>>,
) -> Result<SendResult> {
    let out = if let Some(urls) = to_relays {
        client.send_event_builder_to(urls, builder).await?
    } else {
        client.send_event_builder(builder).await?
    };

    let pubkey = client.signer().await?.get_public_key().await?.to_hex();

    let id = out.id().to_string();
    let success = out.success.into_iter().map(|u| u.to_string()).collect();
    let failed = out
        .failed
        .into_iter()
        .map(|(u, e)| (u.to_string(), e.to_string()))
        .collect();

    Ok(SendResult {
        id,
        success,
        failed,
        pubkey,
    })
}

pub async fn post_text_note(client: &Client, args: PostTextArgs) -> Result<SendResult> {
    let mut builder = EventBuilder::text_note(args.content);
    if let Some(pow) = args.pow {
        builder = builder.pow(pow);
    }
    publish_event_builder(client, builder, args.to_relays).await
}
