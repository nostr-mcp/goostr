use anyhow::Result;
use rmcp::{
    handler::server::tool::ToolRouter,
    model::{Implementation, ProtocolVersion, ServerCapabilities, ServerInfo},
    tool_handler, tool_router,
    transport::stdio,
    ServerHandler, ServiceExt,
};
use tracing::info;

#[derive(Clone)]
struct GoostrServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl GoostrServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_handler]
impl ServerHandler for GoostrServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("Tools: (none)".to_string()),
        }
    }
}

pub async fn start_stdio_server() -> Result<()> {
    let service = GoostrServer::new().serve(stdio()).await?;
    info!("server ready (stdio)");
    service.waiting().await?;
    Ok(())
}
