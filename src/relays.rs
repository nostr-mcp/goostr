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

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PostReactionArgs {
    pub event_id: String,
    pub event_pubkey: String,
    pub content: Option<String>,
    pub event_kind: Option<u16>,
    pub relay_hint: Option<String>,
    pub pow: Option<u8>,
    pub to_relays: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PostCommentArgs {
    pub content: String,
    pub root_event_id: String,
    pub root_event_pubkey: String,
    pub root_event_kind: u16,
    pub parent_event_id: Option<String>,
    pub parent_event_pubkey: Option<String>,
    pub parent_event_kind: Option<u16>,
    pub relay_hint: Option<String>,
    pub pow: Option<u8>,
    pub to_relays: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PostReplyArgs {
    pub content: String,
    pub reply_to_id: String,
    pub reply_to_pubkey: String,
    pub reply_to_kind: u16,
    pub root_event_id: Option<String>,
    pub root_event_pubkey: Option<String>,
    pub mentioned_pubkeys: Option<Vec<String>>,
    pub relay_hint: Option<String>,
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

pub async fn post_reaction(client: &Client, args: PostReactionArgs) -> Result<SendResult> {
    let event_id = EventId::from_hex(&args.event_id)?;
    let event_pubkey = PublicKey::from_hex(&args.event_pubkey)?;

    let content = args.content.unwrap_or_else(|| "+".to_string());

    let event_kind = args.event_kind.map(Kind::from);

    let mut builder = EventBuilder::reaction_extended(event_id, event_pubkey, event_kind, content);

    if let Some(pow) = args.pow {
        builder = builder.pow(pow);
    }

    publish_event_builder(client, builder, args.to_relays).await
}

pub async fn post_reply(client: &Client, args: PostReplyArgs) -> Result<SendResult> {
    let reply_to_kind = args.reply_to_kind;

    if reply_to_kind == 1 {
        post_nip10_reply(client, args).await
    } else {
        post_nip22_comment(client, args).await
    }
}

async fn post_nip10_reply(client: &Client, args: PostReplyArgs) -> Result<SendResult> {
    use crate::error::GoostrError;
    let reply_to_id = EventId::from_hex(&args.reply_to_id)
        .map_err(|e| GoostrError::InvalidEventId(format!("{}: {}", args.reply_to_id, e)))?;

    let reply_to_pubkey = PublicKey::from_hex(&args.reply_to_pubkey)
        .map_err(|e| GoostrError::InvalidPublicKey(format!("{}: {}", args.reply_to_pubkey, e)))?;

    let mut tags = Vec::new();

    let root_id = if let Some(ref root) = args.root_event_id {
        EventId::from_hex(root)
            .map_err(|e| GoostrError::InvalidEventId(format!("{}: {}", root, e)))?
    } else {
        reply_to_id
    };

    let root_pubkey = if let Some(ref root_pk) = args.root_event_pubkey {
        PublicKey::from_hex(root_pk)
            .map_err(|e| GoostrError::InvalidPublicKey(format!("{}: {}", root_pk, e)))?
    } else {
        reply_to_pubkey
    };

    let relay_hint = args.relay_hint.as_deref().unwrap_or("");

    if root_id == reply_to_id {
        tags.push(Tag::custom(
            TagKind::single_letter(Alphabet::E, false),
            vec![
                root_id.to_hex(),
                relay_hint.to_string(),
                "root".to_string(),
                root_pubkey.to_hex(),
            ],
        ));
    } else {
        tags.push(Tag::custom(
            TagKind::single_letter(Alphabet::E, false),
            vec![
                root_id.to_hex(),
                relay_hint.to_string(),
                "root".to_string(),
                root_pubkey.to_hex(),
            ],
        ));

        tags.push(Tag::custom(
            TagKind::single_letter(Alphabet::E, false),
            vec![
                reply_to_id.to_hex(),
                relay_hint.to_string(),
                "reply".to_string(),
                reply_to_pubkey.to_hex(),
            ],
        ));
    }

    tags.push(Tag::custom(
        TagKind::single_letter(Alphabet::P, false),
        vec![reply_to_pubkey.to_hex()],
    ));

    if let Some(mentioned) = args.mentioned_pubkeys {
        for pk_str in mentioned {
            let pk = PublicKey::from_hex(&pk_str)
                .map_err(|e| GoostrError::InvalidPublicKey(format!("{}: {}", pk_str, e)))?;
            tags.push(Tag::custom(
                TagKind::single_letter(Alphabet::P, false),
                vec![pk.to_hex()],
            ));
        }
    }

    let mut builder = EventBuilder::text_note(args.content).tags(tags);

    if let Some(pow) = args.pow {
        builder = builder.pow(pow);
    }

    publish_event_builder(client, builder, args.to_relays).await
}

async fn post_nip22_comment(client: &Client, args: PostReplyArgs) -> Result<SendResult> {
    use crate::error::GoostrError;

    let root_event_id = EventId::from_hex(&args.reply_to_id)
        .map_err(|e| GoostrError::InvalidEventId(format!("{}: {}", args.reply_to_id, e)))?;

    let root_pubkey = PublicKey::from_hex(&args.reply_to_pubkey)
        .map_err(|e| GoostrError::InvalidPublicKey(format!("{}: {}", args.reply_to_pubkey, e)))?;

    let root_kind = Kind::from(args.reply_to_kind);

    let mut tags = Vec::new();

    if let Some(ref relay) = args.relay_hint {
        tags.push(Tag::custom(
            TagKind::single_letter(Alphabet::E, true),
            vec![root_event_id.to_hex(), relay.clone(), root_pubkey.to_hex()],
        ));
    } else {
        tags.push(Tag::custom(
            TagKind::single_letter(Alphabet::E, true),
            vec![root_event_id.to_hex(), String::new(), root_pubkey.to_hex()],
        ));
    }

    tags.push(Tag::custom(
        TagKind::single_letter(Alphabet::K, true),
        vec![root_kind.as_u16().to_string()],
    ));

    if let Some(ref relay) = args.relay_hint {
        tags.push(Tag::custom(
            TagKind::single_letter(Alphabet::P, true),
            vec![root_pubkey.to_hex(), relay.clone()],
        ));
    } else {
        tags.push(Tag::custom(
            TagKind::single_letter(Alphabet::P, true),
            vec![root_pubkey.to_hex()],
        ));
    }

    if let Some(ref relay) = args.relay_hint {
        tags.push(Tag::custom(
            TagKind::single_letter(Alphabet::E, false),
            vec![root_event_id.to_hex(), relay.clone()],
        ));
    } else {
        tags.push(Tag::custom(
            TagKind::single_letter(Alphabet::E, false),
            vec![root_event_id.to_hex()],
        ));
    }

    tags.push(Tag::custom(
        TagKind::Custom(std::borrow::Cow::Borrowed("k")),
        vec![root_kind.as_u16().to_string()],
    ));

    if let Some(ref relay) = args.relay_hint {
        tags.push(Tag::custom(
            TagKind::single_letter(Alphabet::P, false),
            vec![root_pubkey.to_hex(), relay.clone()],
        ));
    } else {
        tags.push(Tag::custom(
            TagKind::single_letter(Alphabet::P, false),
            vec![root_pubkey.to_hex()],
        ));
    }

    let mut builder = EventBuilder::new(Kind::from(1111), args.content).tags(tags);

    if let Some(pow) = args.pow {
        builder = builder.pow(pow);
    }

    publish_event_builder(client, builder, args.to_relays).await
}

pub async fn post_comment(client: &Client, args: PostCommentArgs) -> Result<SendResult> {
    let root_event_id = EventId::from_hex(&args.root_event_id)?;
    let root_pubkey = PublicKey::from_hex(&args.root_event_pubkey)?;
    let root_kind = Kind::from(args.root_event_kind);

    let parent_event_id = if let Some(ref parent_id) = args.parent_event_id {
        EventId::from_hex(parent_id)?
    } else {
        root_event_id
    };

    let parent_pubkey = if let Some(ref parent_pk) = args.parent_event_pubkey {
        PublicKey::from_hex(parent_pk)?
    } else {
        root_pubkey
    };

    let parent_kind = if let Some(parent_k) = args.parent_event_kind {
        Kind::from(parent_k)
    } else {
        root_kind
    };

    let mut tags = Vec::new();

    if let Some(ref relay) = args.relay_hint {
        tags.push(Tag::custom(
            TagKind::single_letter(Alphabet::E, true),
            vec![root_event_id.to_hex(), relay.clone(), root_pubkey.to_hex()],
        ));
    } else {
        tags.push(Tag::custom(
            TagKind::single_letter(Alphabet::E, true),
            vec![root_event_id.to_hex(), String::new(), root_pubkey.to_hex()],
        ));
    }

    tags.push(Tag::custom(
        TagKind::single_letter(Alphabet::K, true),
        vec![root_kind.as_u16().to_string()],
    ));

    if let Some(ref relay) = args.relay_hint {
        tags.push(Tag::custom(
            TagKind::single_letter(Alphabet::P, true),
            vec![root_pubkey.to_hex(), relay.clone()],
        ));
    } else {
        tags.push(Tag::custom(
            TagKind::single_letter(Alphabet::P, true),
            vec![root_pubkey.to_hex()],
        ));
    }

    if let Some(ref relay) = args.relay_hint {
        tags.push(Tag::custom(
            TagKind::single_letter(Alphabet::E, false),
            vec![parent_event_id.to_hex(), relay.clone()],
        ));
    } else {
        tags.push(Tag::custom(
            TagKind::single_letter(Alphabet::E, false),
            vec![parent_event_id.to_hex()],
        ));
    }

    tags.push(Tag::custom(
        TagKind::Custom(std::borrow::Cow::Borrowed("k")),
        vec![parent_kind.as_u16().to_string()],
    ));

    if let Some(ref relay) = args.relay_hint {
        tags.push(Tag::custom(
            TagKind::single_letter(Alphabet::P, false),
            vec![parent_pubkey.to_hex(), relay.clone()],
        ));
    } else {
        tags.push(Tag::custom(
            TagKind::single_letter(Alphabet::P, false),
            vec![parent_pubkey.to_hex()],
        ));
    }

    let mut builder = EventBuilder::new(Kind::from(1111), args.content).tags(tags);

    if let Some(pow) = args.pow {
        builder = builder.pow(pow);
    }

    publish_event_builder(client, builder, args.to_relays).await
}
