use crate::error::Error;
use crate::keys::{
    EmptyArgs, GenerateArgs, ImportArgs, KeyStore, RemoveArgs, RenameLabelArgs, SetActiveArgs,
};
use crate::nostr_client::{ensure_client, reset_cached_client};
use crate::relays::*;
use crate::subscriptions::EventsListArgs;
use crate::util;
use nostr_sdk::prelude::*;
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{
        CallToolResult, Content, ErrorData, Implementation, ProtocolVersion, ServerCapabilities,
        ServerInfo,
    },
    schemars::JsonSchema,
    tool, tool_handler, tool_router,
    transport::stdio,
    ServerHandler, ServiceExt,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};
use tokio::time::{sleep, Duration};
use tracing::info;

static KEYSTORE: OnceCell<RwLock<Arc<KeyStore>>> = OnceCell::const_new();

#[derive(Clone)]
pub struct GoostrServer {
    tool_router: ToolRouter<Self>,
}

impl GoostrServer {
    async fn keystore() -> Result<Arc<KeyStore>, ErrorData> {
        let cell = KEYSTORE
            .get_or_try_init(|| async {
                let path = util::nostr_index_path();
                let ks = KeyStore::load_or_init(path).await?;
                Ok::<RwLock<Arc<KeyStore>>, anyhow::Error>(RwLock::new(Arc::new(ks)))
            })
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        let guard = cell.read().await;
        Ok(guard.clone())
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ConfigDirArgs {
    pub path: Option<String>,
}

#[tool_router]
impl GoostrServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Generate a new Nostr keypair")]
    pub async fn nostr_keys_generate(
        &self,
        Parameters(args): Parameters<GenerateArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let ks = Self::keystore().await?;
        let entry = ks
            .generate(
                args.label,
                args.make_active.unwrap_or(true),
                args.persist_secret.unwrap_or(true),
            )
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        reset_cached_client()
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        let content = Content::json(serde_json::json!(entry))?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "Import secret key (nsec or npub)")]
    pub async fn nostr_keys_import(
        &self,
        Parameters(args): Parameters<ImportArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let ks = Self::keystore().await?;
        let entry = ks
            .import_secret(
                args.label,
                args.key_material,
                args.make_active.unwrap_or(true),
                args.persist_secret.unwrap_or(true),
            )
            .await
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
        reset_cached_client()
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        let content = Content::json(serde_json::json!(entry))?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "Remove a key by label")]
    pub async fn nostr_keys_remove(
        &self,
        Parameters(args): Parameters<RemoveArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let ks = Self::keystore().await?;
        let removed = ks
            .remove(args.label)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        reset_cached_client()
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        let content = Content::json(serde_json::json!({ "removed": removed.is_some() }))?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "List all stored keys (metadata only)")]
    pub async fn nostr_keys_list(
        &self,
        _args: Parameters<EmptyArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let ks = Self::keystore().await?;
        let keys = ks.list().await;
        let active_label = ks.get_active().await.map(|k| k.label);
        let payload =
            serde_json::json!({ "keys": keys, "count": keys.len(), "active": active_label });
        let content = Content::json(payload)?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "Set the active key by label")]
    pub async fn nostr_keys_set_active(
        &self,
        Parameters(args): Parameters<SetActiveArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let ks = Self::keystore().await?;
        let entry = ks
            .set_active(args.label)
            .await
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
        reset_cached_client()
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        let content = Content::json(serde_json::json!(entry))?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "Get the active key (metadata only)")]
    pub async fn nostr_keys_active(
        &self,
        _args: Parameters<EmptyArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let ks = Self::keystore().await?;
        let active = ks.get_active().await;
        let content = Content::json(serde_json::json!(active))?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "Rename a key's label; when 'from' is omitted, renames the active key")]
    pub async fn nostr_keys_rename_label(
        &self,
        Parameters(args): Parameters<RenameLabelArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let ks = Self::keystore().await?;
        let source = match args.from {
            Some(f) => f,
            None => ks
                .get_active()
                .await
                .map(|k| k.label)
                .ok_or_else(|| ErrorData::invalid_params("no active key to rename", None))?,
        };
        let entry = ks
            .rename_label(source, args.to)
            .await
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
        reset_cached_client()
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        let content = Content::json(serde_json::json!(entry))?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "Get or set the directory used to persist the key index (no secrets)")]
    pub async fn nostr_config_dir(
        &self,
        Parameters(args): Parameters<ConfigDirArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        if let Some(p) = args.path {
            std::env::set_var("GOOSTR_DIR", p);
            let path = util::nostr_index_path();
            let new_store = KeyStore::load_or_init(path)
                .await
                .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
            let cell = KEYSTORE
                .get_or_try_init(|| async {
                    let path = util::nostr_index_path();
                    let ks = KeyStore::load_or_init(path).await?;
                    Ok::<RwLock<Arc<KeyStore>>, anyhow::Error>(RwLock::new(Arc::new(ks)))
                })
                .await
                .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
            let mut w = cell.write().await;
            *w = Arc::new(new_store);
            reset_cached_client()
                .await
                .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        }
        let current = util::nostr_config_root();
        let content = Content::json(serde_json::json!({
            "dir": current,
            "file": util::nostr_index_path()
        }))?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(
        description = "Set relays and connect. Requires an active nostr key. read_write: read|write|both"
    )]
    pub async fn nostr_relays_set(
        &self,
        Parameters(args): Parameters<RelaysSetArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let ks = Self::keystore().await?;
        let ac = ensure_client(ks)
            .await
            .map_err(|e: Error| ErrorData::invalid_params(e.to_string(), None))?;
        set_relays(&ac.client, args)
            .await
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
        let rows = list_relays(&ac.client)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        let content = Content::json(serde_json::json!({ "relays": rows }))?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(
        description = "Connect to relays that were previously added. Requires an active nostr key."
    )]
    pub async fn nostr_relays_connect(
        &self,
        Parameters(args): Parameters<RelaysConnectArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let ks = Self::keystore().await?;
        let ac = ensure_client(ks)
            .await
            .map_err(|e: Error| ErrorData::invalid_params(e.to_string(), None))?;
        connect_relays(&ac.client, args)
            .await
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
        let rows = list_relays(&ac.client)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        let content = Content::json(serde_json::json!({ "relays": rows }))?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(
        description = "Disconnect or remove relays. When force_remove=true, relays are removed from the pool."
    )]
    pub async fn nostr_relays_disconnect(
        &self,
        Parameters(args): Parameters<RelaysDisconnectArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let ks = Self::keystore().await?;
        let ac = ensure_client(ks)
            .await
            .map_err(|e: Error| ErrorData::invalid_params(e.to_string(), None))?;
        disconnect_relays(&ac.client, args)
            .await
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
        let rows = list_relays(&ac.client)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        let content = Content::json(serde_json::json!({ "relays": rows }))?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "List relay status and flags")]
    pub async fn nostr_relays_status(
        &self,
        _args: Parameters<EmptyArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let ks = Self::keystore().await?;
        let ac = ensure_client(ks)
            .await
            .map_err(|e: Error| ErrorData::invalid_params(e.to_string(), None))?;
        let rows = list_relays(&ac.client)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        let summary = status_summary(&ac.client)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        let content = Content::json(serde_json::json!({ "summary": summary, "relays": rows }))?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(
        description = "Fetch events using presets. Presets: my_notes, mentions_me, my_metadata, by_author"
    )]
    pub async fn nostr_events_list(
        &self,
        Parameters(args): Parameters<EventsListArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let ks = Self::keystore().await?;
        let ac = ensure_client(ks)
            .await
            .map_err(|e: Error| ErrorData::invalid_params(e.to_string(), None))?;
        let preset = args.preset.to_ascii_lowercase();
        let mut filter = match preset.as_str() {
            "my_notes" => subscription_targets_my_notes(ac.active_pubkey).await,
            "mentions_me" => subscription_targets_mentions_me(ac.active_pubkey).await,
            "my_metadata" => subscription_targets_my_metadata(ac.active_pubkey).await,
            "by_author" => {
                let npub_ref = args.author_npub.as_ref().ok_or_else(|| {
                    ErrorData::invalid_params(
                        "author_npub is required for preset 'by_author'",
                        None,
                    )
                })?;
                let pk = PublicKey::from_bech32(npub_ref)
                    .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
                Filter::new().author(pk).since(Timestamp::now())
            }
            _ => return Err(ErrorData::invalid_params("unknown preset", None)),
        };
        if let Some(l) = args.limit {
            filter = filter.limit(l as usize);
        }
        let events = list_events(&ac.client, filter, args.timeout())
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        let items: Vec<serde_json::Value> = events
            .into_iter()
            .map(|e| {
                serde_json::json!({
                    "id": e.id.to_string(),
                    "kind": e.kind.as_u16(),
                    "pubkey": e.pubkey.to_string(),
                    "created_at": e.created_at.as_u64(),
                    "content": e.content,
                    "tags": e.tags.to_vec(),
                })
            })
            .collect();
        let content = Content::json(serde_json::json!({ "items": items, "count": items.len() }))?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(
        description = "Post a new kind=1 text note to configured relays. Optional: pow (u8), to_relays (urls)"
    )]
    pub async fn nostr_events_post_text(
        &self,
        Parameters(args): Parameters<PostTextArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let ks = Self::keystore().await?;
        let ac = ensure_client(ks)
            .await
            .map_err(|e: Error| ErrorData::invalid_params(e.to_string(), None))?;
        let result = post_text_note(&ac.client, args)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        let content = Content::json(serde_json::json!(result))?;
        Ok(CallToolResult::success(vec![content]))
    }
}

#[tool_handler]
impl ServerHandler for GoostrServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "Tools: nostr_keys_generate, nostr_keys_import, nostr_keys_remove, nostr_keys_list, nostr_keys_set_active, nostr_keys_active, nostr_keys_rename_label, nostr_config_dir, nostr_relays_set, nostr_relays_connect, nostr_relays_disconnect, nostr_relays_status, nostr_events_list, nostr_events_post_text"
                    .to_string(),
            ),
        }
    }
}

async fn wait_for_shutdown() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut term = signal(SignalKind::terminate()).expect("signal");
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {},
            _ = term.recv() => {},
        }
    }

    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }
}

pub async fn start_stdio_server() -> anyhow::Result<()> {
    info!("starting goostr MCP server (stdio)");
    loop {
        let service = GoostrServer::new().serve(stdio()).await?;
        info!("server ready (stdio)");
        tokio::select! {
            _ = service.waiting() => {
                info!("stdio input closed; restarting");
                sleep(Duration::from_millis(200)).await;
                continue;
            }
            _ = wait_for_shutdown() => {
                info!("shutdown signal received");
                break;
            }
        }
    }
    Ok(())
}
