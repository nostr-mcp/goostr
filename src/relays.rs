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

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PostThreadArgs {
    pub content: String,
    pub subject: String,
    pub hashtags: Option<Vec<String>>,
    pub pow: Option<u8>,
    pub to_relays: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PostGroupChatArgs {
    pub content: String,
    pub group_id: String,
    pub reply_to_id: Option<String>,
    pub reply_to_relay: Option<String>,
    pub reply_to_pubkey: Option<String>,
    pub pow: Option<u8>,
    pub to_relays: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct PollOption {
    pub option_id: String,
    pub label: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreatePollArgs {
    pub question: String,
    pub options: Vec<PollOption>,
    pub relay_urls: Vec<String>,
    pub poll_type: Option<String>,
    pub ends_at: Option<u64>,
    pub pow: Option<u8>,
    pub to_relays: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct VotePollArgs {
    pub poll_event_id: String,
    pub option_ids: Vec<String>,
    pub pow: Option<u8>,
    pub to_relays: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetPollResultsArgs {
    pub poll_event_id: String,
    pub timeout_secs: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct PollResultOption {
    pub option_id: String,
    pub label: String,
    pub votes: u64,
}

#[derive(Debug, Serialize)]
pub struct PollResults {
    pub poll_id: String,
    pub question: String,
    pub poll_type: String,
    pub total_votes: u64,
    pub options: Vec<PollResultOption>,
    pub ended: bool,
    pub ends_at: Option<u64>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PutUserArgs {
    pub content: String,
    pub group_id: String,
    pub pubkey: String,
    pub roles: Option<Vec<String>>,
    pub previous_refs: Option<Vec<String>>,
    pub pow: Option<u8>,
    pub to_relays: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RemoveUserArgs {
    pub content: String,
    pub group_id: String,
    pub pubkey: String,
    pub previous_refs: Option<Vec<String>>,
    pub pow: Option<u8>,
    pub to_relays: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EditGroupMetadataArgs {
    pub content: String,
    pub group_id: String,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub about: Option<String>,
    pub public: Option<bool>,
    pub open: Option<bool>,
    pub previous_refs: Option<Vec<String>>,
    pub pow: Option<u8>,
    pub to_relays: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteEventArgs {
    pub content: String,
    pub group_id: String,
    pub event_id: String,
    pub previous_refs: Option<Vec<String>>,
    pub pow: Option<u8>,
    pub to_relays: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateGroupArgs {
    pub content: String,
    pub group_id: String,
    pub previous_refs: Option<Vec<String>>,
    pub pow: Option<u8>,
    pub to_relays: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteGroupArgs {
    pub content: String,
    pub group_id: String,
    pub previous_refs: Option<Vec<String>>,
    pub pow: Option<u8>,
    pub to_relays: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateInviteArgs {
    pub content: String,
    pub group_id: String,
    pub previous_refs: Option<Vec<String>>,
    pub pow: Option<u8>,
    pub to_relays: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct JoinGroupArgs {
    pub content: String,
    pub group_id: String,
    pub invite_code: Option<String>,
    pub pow: Option<u8>,
    pub to_relays: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LeaveGroupArgs {
    pub content: String,
    pub group_id: String,
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

pub async fn post_thread(client: &Client, args: PostThreadArgs) -> Result<SendResult> {
    let mut tags = Vec::new();

    tags.push(Tag::parse(&["subject".to_string(), args.subject.clone()])?);

    if let Some(hashtags) = args.hashtags {
        for hashtag in hashtags {
            tags.push(Tag::parse(&["t".to_string(), hashtag])?);
        }
    }

    let mut builder = EventBuilder::new(Kind::from(11), args.content).tags(tags);

    if let Some(pow) = args.pow {
        builder = builder.pow(pow);
    }

    publish_event_builder(client, builder, args.to_relays).await
}

pub async fn post_group_chat(
    client: &Client,
    args: PostGroupChatArgs,
) -> Result<SendResult> {
    use crate::error::GoostrError;
    
    let mut tags = Vec::new();

    tags.push(Tag::parse(&["h".to_string(), args.group_id.clone()])?);

    if let Some(ref reply_id) = args.reply_to_id {
        let event_id = EventId::from_hex(reply_id)
            .map_err(|e| GoostrError::InvalidEventId(format!("{}: {}", reply_id, e)))?;
        
        let relay = args.reply_to_relay.as_deref().unwrap_or("");
        let pubkey = args.reply_to_pubkey.as_deref().unwrap_or("");
        
        tags.push(Tag::parse(&[
            "q".to_string(),
            event_id.to_hex(),
            relay.to_string(),
            pubkey.to_string(),
        ])?);
    }

    let mut builder = EventBuilder::new(Kind::from(9), args.content).tags(tags);

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

    let relay_hint = if let Some(relay) = args.relay_hint {
        Some(RelayUrl::parse(&relay).map_err(|e| {
            crate::error::GoostrError::Invalid(format!("invalid relay url: {e}"))
        })?)
    } else {
        None
    };

    let target = ReactionTarget {
        event_id,
        public_key: event_pubkey,
        coordinate: None,
        kind: event_kind,
        relay_hint,
    };

    let mut builder = EventBuilder::reaction(target, content);

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

pub async fn create_poll(client: &Client, args: CreatePollArgs) -> Result<SendResult> {
    use crate::error::GoostrError;

    if args.options.len() < 2 {
        return Err(GoostrError::Invalid("poll must have at least 2 options".to_string()).into());
    }

    let mut option_ids = std::collections::HashSet::new();
    for option in &args.options {
        if !option_ids.insert(&option.option_id) {
            return Err(GoostrError::Invalid(format!(
                "duplicate option ID: {}",
                option.option_id
            ))
            .into());
        }
    }

    let mut tags = Vec::new();

    for option in &args.options {
        tags.push(Tag::parse(&[
            "option".to_string(),
            option.option_id.clone(),
            option.label.clone(),
        ])?);
    }

    for relay_url in &args.relay_urls {
        tags.push(Tag::parse(&["relay".to_string(), relay_url.clone()])?);
    }

    let poll_type = args.poll_type.as_deref().unwrap_or("singlechoice");
    tags.push(Tag::parse(&["polltype".to_string(), poll_type.to_string()])?);

    if let Some(ends_at) = args.ends_at {
        tags.push(Tag::parse(&["endsAt".to_string(), ends_at.to_string()])?);
    }

    let mut builder = EventBuilder::new(Kind::from(1068), args.question).tags(tags);

    if let Some(pow) = args.pow {
        builder = builder.pow(pow);
    }

    publish_event_builder(client, builder, args.to_relays).await
}

pub async fn vote_poll(client: &Client, args: VotePollArgs) -> Result<SendResult> {
    use crate::error::GoostrError;

    if args.option_ids.is_empty() {
        return Err(GoostrError::Invalid("must select at least one option".to_string()).into());
    }

    let poll_event_id = EventId::from_hex(&args.poll_event_id)
        .map_err(|e| GoostrError::InvalidEventId(format!("{}: {}", args.poll_event_id, e)))?;

    let mut tags = Vec::new();

    tags.push(Tag::parse(&["e".to_string(), poll_event_id.to_hex()])?);

    for option_id in &args.option_ids {
        tags.push(Tag::parse(&["response".to_string(), option_id.clone()])?);
    }

    let mut builder = EventBuilder::new(Kind::from(1018), "").tags(tags);

    if let Some(pow) = args.pow {
        builder = builder.pow(pow);
    }

    publish_event_builder(client, builder, args.to_relays).await
}

pub async fn get_poll_results(
    client: &Client,
    poll_event_id: &str,
    timeout_secs: u64,
) -> Result<PollResults> {
    use crate::error::GoostrError;

    let poll_id = EventId::from_hex(poll_event_id)
        .map_err(|e| GoostrError::InvalidEventId(format!("{}: {}", poll_event_id, e)))?;

    let poll_filter = Filter::new().id(poll_id).kind(Kind::from(1068)).limit(1);

    let poll_events = client
        .fetch_events(poll_filter, std::time::Duration::from_secs(timeout_secs))
        .await?;

    let poll_event = poll_events
        .iter()
        .next()
        .ok_or_else(|| GoostrError::Invalid("poll not found".to_string()))?;

    let mut options_map: HashMap<String, String> = HashMap::new();
    let mut relay_urls = Vec::new();
    let mut poll_type = "singlechoice".to_string();
    let mut ends_at: Option<u64> = None;

    for tag in poll_event.tags.iter() {
        let tag_vec = tag.clone().to_vec();
        if tag_vec.is_empty() {
            continue;
        }

        match tag_vec[0].as_str() {
            "option" if tag_vec.len() >= 3 => {
                options_map.insert(tag_vec[1].clone(), tag_vec[2].clone());
            }
            "relay" if tag_vec.len() >= 2 => {
                relay_urls.push(tag_vec[1].clone());
            }
            "polltype" if tag_vec.len() >= 2 => {
                poll_type = tag_vec[1].clone();
            }
            "endsAt" if tag_vec.len() >= 2 => {
                if let Ok(timestamp) = tag_vec[1].parse::<u64>() {
                    ends_at = Some(timestamp);
                }
            }
            _ => {}
        }
    }

    let vote_filter = Filter::new()
        .kind(Kind::from(1018))
        .custom_tag(SingleLetterTag::lowercase(Alphabet::E), poll_id.to_hex());

    let vote_events = client
        .fetch_events(vote_filter, std::time::Duration::from_secs(timeout_secs))
        .await?;

    let mut vote_counts: HashMap<String, u64> = HashMap::new();
    let mut votes_by_pubkey: HashMap<String, (u64, Vec<String>)> = HashMap::new();

    let now = Timestamp::now().as_secs();
    let ended = ends_at.map_or(false, |end_time| now > end_time);

    for vote_event in vote_events.iter() {
        let vote_time = vote_event.created_at.as_secs();

        if let Some(end_time) = ends_at {
            if vote_time > end_time {
                continue;
            }
        }

        let pubkey = vote_event.pubkey.to_hex();
        let mut selected_options = Vec::new();

        for tag in vote_event.tags.iter() {
            let tag_vec = tag.clone().to_vec();
            if tag_vec.len() >= 2 && tag_vec[0] == "response" {
                selected_options.push(tag_vec[1].clone());
            }
        }

        if selected_options.is_empty() {
            continue;
        }

        if let Some((existing_time, _)) = votes_by_pubkey.get(&pubkey) {
            if vote_time <= *existing_time {
                continue;
            }
        }

        votes_by_pubkey.insert(pubkey, (vote_time, selected_options));
    }

    for (_, (_time, selected_options)) in votes_by_pubkey.iter() {
        for option_id in selected_options {
            if options_map.contains_key(option_id) {
                *vote_counts.entry(option_id.clone()).or_insert(0) += 1;
            }
        }
    }

    let total_votes = votes_by_pubkey.len() as u64;

    let mut options: Vec<PollResultOption> = options_map
        .into_iter()
        .map(|(option_id, label)| PollResultOption {
            option_id: option_id.clone(),
            label,
            votes: *vote_counts.get(&option_id).unwrap_or(&0),
        })
        .collect();

    options.sort_by(|a, b| a.option_id.cmp(&b.option_id));

    Ok(PollResults {
        poll_id: poll_event_id.to_string(),
        question: poll_event.content.clone(),
        poll_type,
        total_votes,
        options,
        ended,
        ends_at,
    })
}

pub async fn put_user(client: &Client, args: PutUserArgs) -> Result<SendResult> {
    use crate::error::GoostrError;
    
    let pubkey = PublicKey::from_hex(&args.pubkey)
        .map_err(|e| GoostrError::InvalidPublicKey(format!("{}: {}", args.pubkey, e)))?;

    let mut tags = Vec::new();

    tags.push(Tag::parse(&["h".to_string(), args.group_id.clone()])?);

    let mut p_tag = vec!["p".to_string(), pubkey.to_hex()];
    if let Some(roles) = args.roles {
        for role in roles {
            p_tag.push(role);
        }
    }
    tags.push(Tag::parse(&p_tag)?);

    if let Some(refs) = args.previous_refs {
        for ref_id in refs {
            tags.push(Tag::parse(&["previous".to_string(), ref_id])?);
        }
    }

    let mut builder = EventBuilder::new(Kind::from(9000), args.content).tags(tags);

    if let Some(pow) = args.pow {
        builder = builder.pow(pow);
    }

    publish_event_builder(client, builder, args.to_relays).await
}

pub async fn remove_user(client: &Client, args: RemoveUserArgs) -> Result<SendResult> {
    use crate::error::GoostrError;
    
    let pubkey = PublicKey::from_hex(&args.pubkey)
        .map_err(|e| GoostrError::InvalidPublicKey(format!("{}: {}", args.pubkey, e)))?;

    let mut tags = Vec::new();

    tags.push(Tag::parse(&["h".to_string(), args.group_id.clone()])?);
    tags.push(Tag::parse(&["p".to_string(), pubkey.to_hex()])?);

    if let Some(refs) = args.previous_refs {
        for ref_id in refs {
            tags.push(Tag::parse(&["previous".to_string(), ref_id])?);
        }
    }

    let mut builder = EventBuilder::new(Kind::from(9001), args.content).tags(tags);

    if let Some(pow) = args.pow {
        builder = builder.pow(pow);
    }

    publish_event_builder(client, builder, args.to_relays).await
}

pub async fn edit_group_metadata(client: &Client, args: EditGroupMetadataArgs) -> Result<SendResult> {
    let mut tags = Vec::new();

    tags.push(Tag::parse(&["h".to_string(), args.group_id.clone()])?);

    if let Some(name) = args.name {
        tags.push(Tag::parse(&["name".to_string(), name])?);
    }

    if let Some(picture) = args.picture {
        tags.push(Tag::parse(&["picture".to_string(), picture])?);
    }

    if let Some(about) = args.about {
        tags.push(Tag::parse(&["about".to_string(), about])?);
    }

    if let Some(public) = args.public {
        if public {
            tags.push(Tag::parse(&["public".to_string()])?);
        } else {
            tags.push(Tag::parse(&["private".to_string()])?);
        }
    }

    if let Some(open) = args.open {
        if open {
            tags.push(Tag::parse(&["open".to_string()])?);
        } else {
            tags.push(Tag::parse(&["closed".to_string()])?);
        }
    }

    if let Some(refs) = args.previous_refs {
        for ref_id in refs {
            tags.push(Tag::parse(&["previous".to_string(), ref_id])?);
        }
    }

    let mut builder = EventBuilder::new(Kind::from(9002), args.content).tags(tags);

    if let Some(pow) = args.pow {
        builder = builder.pow(pow);
    }

    publish_event_builder(client, builder, args.to_relays).await
}

pub async fn delete_group_event(client: &Client, args: DeleteEventArgs) -> Result<SendResult> {
    use crate::error::GoostrError;
    
    let event_id = EventId::from_hex(&args.event_id)
        .map_err(|e| GoostrError::InvalidEventId(format!("{}: {}", args.event_id, e)))?;

    let mut tags = Vec::new();

    tags.push(Tag::parse(&["h".to_string(), args.group_id.clone()])?);
    tags.push(Tag::parse(&["e".to_string(), event_id.to_hex()])?);

    if let Some(refs) = args.previous_refs {
        for ref_id in refs {
            tags.push(Tag::parse(&["previous".to_string(), ref_id])?);
        }
    }

    let mut builder = EventBuilder::new(Kind::from(9005), args.content).tags(tags);

    if let Some(pow) = args.pow {
        builder = builder.pow(pow);
    }

    publish_event_builder(client, builder, args.to_relays).await
}

pub async fn create_group(client: &Client, args: CreateGroupArgs) -> Result<SendResult> {
    let mut tags = Vec::new();

    tags.push(Tag::parse(&["h".to_string(), args.group_id.clone()])?);

    if let Some(refs) = args.previous_refs {
        for ref_id in refs {
            tags.push(Tag::parse(&["previous".to_string(), ref_id])?);
        }
    }

    let mut builder = EventBuilder::new(Kind::from(9007), args.content).tags(tags);

    if let Some(pow) = args.pow {
        builder = builder.pow(pow);
    }

    publish_event_builder(client, builder, args.to_relays).await
}

pub async fn delete_group(client: &Client, args: DeleteGroupArgs) -> Result<SendResult> {
    let mut tags = Vec::new();

    tags.push(Tag::parse(&["h".to_string(), args.group_id.clone()])?);

    if let Some(refs) = args.previous_refs {
        for ref_id in refs {
            tags.push(Tag::parse(&["previous".to_string(), ref_id])?);
        }
    }

    let mut builder = EventBuilder::new(Kind::from(9008), args.content).tags(tags);

    if let Some(pow) = args.pow {
        builder = builder.pow(pow);
    }

    publish_event_builder(client, builder, args.to_relays).await
}

pub async fn create_invite(client: &Client, args: CreateInviteArgs) -> Result<SendResult> {
    let mut tags = Vec::new();

    tags.push(Tag::parse(&["h".to_string(), args.group_id.clone()])?);

    if let Some(refs) = args.previous_refs {
        for ref_id in refs {
            tags.push(Tag::parse(&["previous".to_string(), ref_id])?);
        }
    }

    let mut builder = EventBuilder::new(Kind::from(9009), args.content).tags(tags);

    if let Some(pow) = args.pow {
        builder = builder.pow(pow);
    }

    publish_event_builder(client, builder, args.to_relays).await
}

pub async fn join_group(client: &Client, args: JoinGroupArgs) -> Result<SendResult> {
    let mut tags = Vec::new();

    tags.push(Tag::parse(&["h".to_string(), args.group_id.clone()])?);

    if let Some(code) = args.invite_code {
        tags.push(Tag::parse(&["code".to_string(), code])?);
    }

    let mut builder = EventBuilder::new(Kind::from(9021), args.content).tags(tags);

    if let Some(pow) = args.pow {
        builder = builder.pow(pow);
    }

    publish_event_builder(client, builder, args.to_relays).await
}

pub async fn leave_group(client: &Client, args: LeaveGroupArgs) -> Result<SendResult> {
    let mut tags = Vec::new();

    tags.push(Tag::parse(&["h".to_string(), args.group_id.clone()])?);

    let mut builder = EventBuilder::new(Kind::from(9022), args.content).tags(tags);

    if let Some(pow) = args.pow {
        builder = builder.pow(pow);
    }

    publish_event_builder(client, builder, args.to_relays).await
}
