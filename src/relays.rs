use anyhow::Result;
pub use nostr_mcp_core::relays::{
    RelaysConnectArgs, RelaysDisconnectArgs, RelaysSetArgs, RelayStatusRow,
};
use nostr_mcp_core::publish as core_publish;
use nostr_mcp_core::relays as core_relays;
use nostr_mcp_core::events as core_events;
use nostr_mcp_core::replies as core_replies;
use nostr_mcp_core::polls as core_polls;
use nostr_mcp_core::groups as core_groups;
use nostr_sdk::prelude::*;
use std::collections::HashMap;

pub use nostr_mcp_core::publish::{PostGroupChatArgs, PostReactionArgs, PostTextArgs, PostThreadArgs, SendResult};
pub use nostr_mcp_core::replies::{PostCommentArgs, PostReplyArgs};
pub use nostr_mcp_core::polls::{
    CreatePollArgs, GetPollResultsArgs, PollOption, PollResultOption, PollResults, VotePollArgs,
};
pub use nostr_mcp_core::groups::{
    CreateGroupArgs, CreateInviteArgs, DeleteEventArgs, DeleteGroupArgs, EditGroupMetadataArgs,
    JoinGroupArgs, LeaveGroupArgs, PutUserArgs, RemoveUserArgs,
};

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
    core_groups::put_user(client, args)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn remove_user(client: &Client, args: RemoveUserArgs) -> Result<SendResult> {
    core_groups::remove_user(client, args)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn edit_group_metadata(client: &Client, args: EditGroupMetadataArgs) -> Result<SendResult> {
    core_groups::edit_group_metadata(client, args)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn delete_group_event(client: &Client, args: DeleteEventArgs) -> Result<SendResult> {
    core_groups::delete_group_event(client, args)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn create_group(client: &Client, args: CreateGroupArgs) -> Result<SendResult> {
    core_groups::create_group(client, args)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn delete_group(client: &Client, args: DeleteGroupArgs) -> Result<SendResult> {
    core_groups::delete_group(client, args)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn create_invite(client: &Client, args: CreateInviteArgs) -> Result<SendResult> {
    core_groups::create_invite(client, args)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn join_group(client: &Client, args: JoinGroupArgs) -> Result<SendResult> {
    core_groups::join_group(client, args)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn leave_group(client: &Client, args: LeaveGroupArgs) -> Result<SendResult> {
    core_groups::leave_group(client, args)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}
