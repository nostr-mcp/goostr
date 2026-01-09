pub async fn start_stdio_server() -> anyhow::Result<()> {
    nostr_mcp_tools::server::start_stdio_server().await
}
