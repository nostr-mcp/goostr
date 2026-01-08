use anyhow::Result;
pub use nostr_mcp_core::relays::{
    RelaysConnectArgs, RelaysDisconnectArgs, RelaysSetArgs, RelayStatusRow,
};
use nostr_mcp_core::publish as core_publish;
use nostr_mcp_core::relays as core_relays;
use nostr_mcp_core::events as core_events;
use nostr_mcp_core::replies as core_replies;
use nostr_sdk::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use nostr_mcp_core::publish::{PostGroupChatArgs, PostReactionArgs, PostTextArgs, PostThreadArgs, SendResult};
pub use nostr_mcp_core::replies::{PostCommentArgs, PostReplyArgs};

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
    core_relays::set_relays(client, args)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn connect_relays(client: &Client, args: RelaysConnectArgs) -> Result<()> {
    core_relays::connect_relays(client, args)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn disconnect_relays(client: &Client, args: RelaysDisconnectArgs) -> Result<()> {
    core_relays::disconnect_relays(client, args)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn list_relays(client: &Client) -> Result<Vec<RelayStatusRow>> {
    core_relays::list_relays(client)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn get_relay_urls(client: &Client) -> Vec<String> {
    core_relays::get_relay_urls(client).await
}

pub async fn subscription_targets_my_notes(
    pk: PublicKey,
    since: Option<Timestamp>,
    until: Option<Timestamp>,
) -> Filter {
    core_events::subscription_targets_my_notes(pk, since, until).await
}

pub async fn subscription_targets_mentions_me(
    pk: PublicKey,
    since: Option<Timestamp>,
    until: Option<Timestamp>,
) -> Filter {
    core_events::subscription_targets_mentions_me(pk, since, until).await
}

pub async fn subscription_targets_my_metadata(pk: PublicKey) -> Filter {
    core_events::subscription_targets_my_metadata(pk).await
}

pub async fn list_events(client: &Client, filter: Filter, timeout_secs: u64) -> Result<Vec<Event>> {
    core_events::list_events(client, filter, timeout_secs)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn status_summary(client: &Client) -> Result<HashMap<String, String>> {
    core_relays::status_summary(client)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn publish_event_builder(
    client: &Client,
    builder: EventBuilder,
    to_relays: Option<Vec<String>>,
) -> Result<SendResult> {
    core_publish::publish_event_builder(client, builder, to_relays)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn post_text_note(client: &Client, args: PostTextArgs) -> Result<SendResult> {
    core_publish::post_text_note(client, args)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn post_thread(client: &Client, args: PostThreadArgs) -> Result<SendResult> {
    core_publish::post_thread(client, args)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn post_group_chat(
    client: &Client,
    args: PostGroupChatArgs,
) -> Result<SendResult> {
    core_publish::post_group_chat(client, args)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn post_reaction(client: &Client, args: PostReactionArgs) -> Result<SendResult> {
    core_publish::post_reaction(client, args)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn post_reply(client: &Client, args: PostReplyArgs) -> Result<SendResult> {
    core_replies::post_reply(client, args)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn post_comment(client: &Client, args: PostCommentArgs) -> Result<SendResult> {
    core_replies::post_comment(client, args)
        .await
        .map_err(|e| anyhow::anyhow!(e))
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
