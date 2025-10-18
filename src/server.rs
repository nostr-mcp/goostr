use crate::keys::{
    EmptyArgs, GenerateArgs, ImportArgs, KeyStore, RemoveArgs, RenameLabelArgs, SetActiveArgs,
};
use crate::util;
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
        }
        let current = util::nostr_config_root();
        let content = Content::json(serde_json::json!({
            "dir": current,
            "file": util::nostr_index_path()
        }))?;
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
                "Tools: nostr_keys_generate, nostr_keys_import, nostr_keys_remove, nostr_keys_list, nostr_keys_set_active, nostr_keys_active, nostr_keys_rename_label, nostr_config_dir"
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
