use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::router::tool::ToolRouter,
    model::*,
    tool, tool_handler, tool_router,
    transport::streamable_http_server::{
        StreamableHttpService, session::local::LocalSessionManager,
    },
};
use tracing_subscriber::prelude::*;

#[derive(Clone)]
pub struct AuroraMcpServer {
    tool_router: ToolRouter<AuroraMcpServer>,
}

impl AuroraMcpServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router]
impl AuroraMcpServer {
    #[tool(description = "Return a hello world greeting")]
    fn hello_world(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text(
            "Hello, world from Aurora-RS-MCP!",
        )]))
    }

    #[tool(description = "Get system information from Aurora OS")]
    fn get_aurora_info(&self) -> Result<CallToolResult, McpError> {
        let device_model = crate::dbus::dbus_deviceinfo::ac_device_info_get_device_model();
        let cache_location = crate::clib::c_appdir::get_app_cache_location();

        let info = format!(
            "Aurora OS System Info:\n\
             - Device Model: {:?}\n\
             - Cache Location: {:?}",
            device_model, cache_location
        );

        Ok(CallToolResult::success(vec![Content::text(info)]))
    }
}

#[tool_handler]
impl ServerHandler for AuroraMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "Aurora-RS-MCP exposes MCP tools for demonstration:\n\
                 - hello_world: Simple greeting tool\n\
                 - get_aurora_info: Retrieve device and cache info".to_string()
            ),
        }
    }
}

pub async fn start_mcp_server() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Aurora-RS-MCP server...");

    // Create service with session management
    let service = StreamableHttpService::new(
        || Ok(AuroraMcpServer::new()),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    // Setup HTTP router
    let router = axum::Router::new().nest_service("/mcp", service);

    const BIND_ADDRESS: &str = "0.0.0.0:8080";
    tracing::info!("MCP server listening on http://{}", BIND_ADDRESS);

    let tcp_listener = tokio::net::TcpListener::bind(BIND_ADDRESS).await?;

    // Start the server with graceful shutdown
    println!("Aurora-RS-MCP server is running. Waiting for connections...");

    let server_handle = axum::serve(tcp_listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await;

    match server_handle {
        Ok(_) => {
            println!("Aurora-RS-MCP server stopped gracefully");
            tracing::info!("Aurora-RS-MCP server stopped gracefully");
        },
        Err(e) => {
            eprintln!("Server error: {}", e);
            tracing::error!("Server error: {}", e);
        }
    }

    Ok(())
}

async fn shutdown_signal() {
    // Wait for Ctrl+C
    tokio::signal::ctrl_c().await
        .expect("Failed to install Ctrl+C handler");

    println!("\nReceived shutdown signal, shutting down gracefully...");
    tracing::info!("Received shutdown signal, shutting down gracefully...");
}
