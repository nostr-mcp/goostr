use anyhow::Result;
pub use nostr_mcp_core::relays::{
    RelaysConnectArgs, RelaysDisconnectArgs, RelaysSetArgs, RelayStatusRow,
};
use nostr_mcp_core::publish as core_publish;
use nostr_mcp_core::relays as core_relays;
use nostr_mcp_core::events as core_events;
use nostr_mcp_core::replies as core_replies;
use nostr_mcp_core::polls as core_polls;
use nostr_sdk::prelude::*;
use schemars::JsonSchema;
use serde::Deserialize;
use std::collections::HashMap;

pub use nostr_mcp_core::publish::{PostGroupChatArgs, PostReactionArgs, PostTextArgs, PostThreadArgs, SendResult};
pub use nostr_mcp_core::replies::{PostCommentArgs, PostReplyArgs};
pub use nostr_mcp_core::polls::{
    CreatePollArgs, GetPollResultsArgs, PollOption, PollResultOption, PollResults, VotePollArgs,
};

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
    core_polls::create_poll(client, args)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn vote_poll(client: &Client, args: VotePollArgs) -> Result<SendResult> {
    core_polls::vote_poll(client, args)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn get_poll_results(
    client: &Client,
    poll_event_id: &str,
    timeout_secs: u64,
) -> Result<PollResults> {
    core_polls::get_poll_results(client, poll_event_id, timeout_secs)
        .await
        .map_err(|e| anyhow::anyhow!(e))
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
