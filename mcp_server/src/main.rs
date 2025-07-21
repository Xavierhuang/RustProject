use rmcp::{tool, tool_router, server::{McpServer, ToolRouter, ServerHandler}, model::{CallToolResult, Content, ServerInfo, ServerCapabilities}};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct BlockchainTools {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl BlockchainTools {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Send ETH to a recipient address")]
    async fn send_eth(&self, amount: f64, recipient: String) -> Result<CallToolResult, rmcp::ErrorData> {
        // ... your logic here ...
        Ok(CallToolResult::success(vec![Content::text(format!("Would send {} ETH to {}", amount, recipient))]))
    }
}

#[rmcp::tool_handler]
impl ServerHandler for BlockchainTools {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Blockchain tool server".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:3000";
    let service = BlockchainTools::new();
    let server = McpServer::new(service);
    println!("RMCP server running at http://{}", addr);
    server.run(addr).await?;
    Ok(())
}
