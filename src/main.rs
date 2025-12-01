use clap::Parser;

mod clib;
mod cxx;
mod dbus;
mod mcp_server;

const DEFAULT_PORT: u16 = 8080;

/// Minimal MCP server for Aurora OS
#[derive(Parser)]
#[command(name = "aurora-rs-mcp")]
struct Cli {
    /// TCP port for the MCP HTTP listener
    #[arg(long = "port", default_value_t = DEFAULT_PORT)]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let port = args.port;

    println!("Aurora-RS-MCP - Starting MCP Server for Aurora OS...");
    println!("Server will be available at: http://0.0.0.0:{}/mcp", port);
    println!("Press Ctrl+C to stop the server");

    mcp_server::start_mcp_server(port).await
}
