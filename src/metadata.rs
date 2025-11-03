use crate::settings::ProfileMetadata;
use anyhow::{Context, Result};
use nostr_sdk::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetMetadataArgs {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub about: Option<String>,
    pub picture: Option<String>,
    pub banner: Option<String>,
    pub nip05: Option<String>,
    pub lud06: Option<String>,
    pub lud16: Option<String>,
    pub website: Option<String>,
    pub publish: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct FetchMetadataArgs {
    pub label: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MetadataResult {
    pub saved: bool,
    pub published: bool,
    pub event_id: Option<String>,
    pub pubkey: Option<String>,
    pub success_relays: Vec<String>,
    pub failed_relays: HashMap<String, String>,
}

pub fn profile_to_nostr_metadata(profile: &ProfileMetadata) -> Result<Metadata> {
    let mut metadata = Metadata::new();

    if let Some(name) = &profile.name {
        metadata = metadata.name(name);
    }
    if let Some(display_name) = &profile.display_name {
        metadata = metadata.display_name(display_name);
    }
    if let Some(about) = &profile.about {
        metadata = metadata.about(about);
    }
    if let Some(picture) = &profile.picture {
        let url = Url::parse(picture).context("invalid picture URL")?;
        metadata = metadata.picture(url);
    }
    if let Some(banner) = &profile.banner {
        let url = Url::parse(banner).context("invalid banner URL")?;
        metadata = metadata.banner(url);
    }
    if let Some(nip05) = &profile.nip05 {
        metadata = metadata.nip05(nip05);
    }
    if let Some(lud06) = &profile.lud06 {
        metadata = metadata.lud06(lud06);
    }
    if let Some(lud16) = &profile.lud16 {
        metadata = metadata.lud16(lud16);
    }
    if let Some(website) = &profile.website {
        let url = Url::parse(website).context("invalid website URL")?;
        metadata = metadata.website(url);
    }

    Ok(metadata)
}

pub fn args_to_profile(args: &SetMetadataArgs) -> ProfileMetadata {
    ProfileMetadata {
        name: args.name.clone(),
        display_name: args.display_name.clone(),
        about: args.about.clone(),
        picture: args.picture.clone(),
        banner: args.banner.clone(),
        nip05: args.nip05.clone(),
        lud06: args.lud06.clone(),
        lud16: args.lud16.clone(),
        website: args.website.clone(),
    }
}

pub async fn publish_metadata(
    client: &Client,
    profile: &ProfileMetadata,
) -> Result<MetadataResult> {
    let metadata = profile_to_nostr_metadata(profile)?;
    let builder = EventBuilder::metadata(&metadata);

    let output = client.send_event_builder(builder).await?;

    let pubkey = client.signer().await?.get_public_key().await?.to_hex();

    let event_id = output.id().to_string();
    let success_relays: Vec<String> = output.success.iter().map(|u| u.to_string()).collect();
    let failed_relays: HashMap<String, String> = output
        .failed
        .iter()
        .map(|(u, e)| (u.to_string(), e.to_string()))
        .collect();

    Ok(MetadataResult {
        saved: true,
        published: true,
        event_id: Some(event_id),
        pubkey: Some(pubkey),
        success_relays,
        failed_relays,
    })
}

pub async fn fetch_metadata(client: &Client, pubkey: &PublicKey) -> Result<Option<Metadata>> {
    let filter = Filter::new().author(*pubkey).kind(Kind::Metadata).limit(1);

    let events = client
        .fetch_events(filter, std::time::Duration::from_secs(10))
        .await?;

    if let Some(event) = events.first() {
        let metadata = Metadata::from_json(&event.content)?;
        Ok(Some(metadata))
    } else {
        Ok(None)
    }
}
