use axum::{routing::post, Router, Json};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use ethers::prelude::*;
use std::str::FromStr;

#[derive(Deserialize)]
struct CommandRequest {
    command: String,
}

#[derive(Serialize)]
struct CommandResponse {
    response: String,
}

async fn handle_command(Json(payload): Json<CommandRequest>) -> Json<CommandResponse> {
    let command = payload.command.trim();
    // Simple parse: send <amount> ETH from my address to <recipient>
    let lower = command.to_lowercase();
    if lower.starts_with("send ") && lower.contains("eth from my address to ") {
        // Example: send 1 ETH from my address to 0xAlice
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.len() < 8 {
            return Json(CommandResponse {
                response: "Invalid command format.".to_string(),
            });
        }
        // Parse amount
        let amount_str = parts[1];
        let amount: f64 = match amount_str.parse() {
            Ok(a) => a,
            Err(_) => {
                return Json(CommandResponse {
                    response: "Invalid amount.".to_string(),
                });
            }
        };
        // Parse recipient
        let recipient_str = parts[7];
        let recipient = match Address::from_str(recipient_str) {
            Ok(addr) => addr,
            Err(_) => {
                return Json(CommandResponse {
                    response: "Invalid recipient address.".to_string(),
                });
            }
        };
        // Default sender: Anvil account 0
        let private_key = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let provider = match Provider::<Http>::try_from("http://127.0.0.1:8545") {
            Ok(p) => p,
            Err(e) => {
                return Json(CommandResponse {
                    response: format!("Provider error: {}", e),
                });
            }
        };
        let chain_id = 31337u64; // Anvil default
        let wallet = match LocalWallet::from_str(private_key) {
            Ok(w) => w.with_chain_id(chain_id),
            Err(e) => {
                return Json(CommandResponse {
                    response: format!("Wallet error: {}", e),
                });
            }
        };
        let client = SignerMiddleware::new(provider, wallet);
        let address = client.address(); // Get the address before moving client
        let client = NonceManagerMiddleware::new(client, address);
        let client = std::sync::Arc::new(client);
        // Convert ETH to wei
        let wei = U256::from_dec_str(&((amount * 1e18) as u128).to_string()).unwrap();
        // Send transaction
        let tx = TransactionRequest::pay(recipient, wei);
        let pending = match client.send_transaction(tx, None).await {
            Ok(p) => p,
            Err(e) => {
                return Json(CommandResponse {
                    response: format!("Transaction error: {}", e),
                });
            }
        };
        let tx_hash = *pending; // Get the transaction hash immediately
        return Json(CommandResponse {
            response: format!("Transaction sent! Hash: {:#x}", tx_hash),
        });
    }
    // Fallback: echo
    Json(CommandResponse {
        response: format!("Echo: {}", command),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/command", post(handle_command));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("MCP server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
