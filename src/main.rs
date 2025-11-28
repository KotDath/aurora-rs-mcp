mod clib;
mod cxx;
mod dbus;
mod mcp_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Aurora-RS-MCP - Starting MCP Server for Aurora OS...");
    println!("Server will be available at: http://127.0.0.1:8080/mcp");
    println!("Press Ctrl+C to stop the server");

    // Start MCP server and run indefinitely
    mcp_server::start_mcp_server().await
}
