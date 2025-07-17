use axum::{routing::post, Router, Json};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use reqwest::Client;
use std::env;

#[derive(Deserialize)]
struct CommandRequest {
    command: String,
}

#[derive(Serialize)]
struct CommandResponse {
    response: String,
}

#[derive(Serialize)]
struct ResetResponse {
    status: String,
}

async fn call_anthropic(prompt: &str) -> Result<String, String> {
    let api_key = env::var("ANTHROPIC_API_KEY").map_err(|_| "Missing ANTHROPIC_API_KEY env var".to_string())?;
    let client = Client::new();
    let url = "https://api.anthropic.com/v1/messages";
    let body = serde_json::json!({
        "model": "claude-3-5-haiku-latest",
        "max_tokens": 256,
        "messages": [
            {"role": "user", "content": prompt}
        ]
    });
    let resp = client.post(url)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Anthropic API error: {}", e))?;
    let json: serde_json::Value = resp.json().await.map_err(|e| format!("Anthropic JSON error: {}", e))?;
    println!("Anthropic raw response: {}", json); // Debug print
    let content = json["content"][0]["text"].as_str().unwrap_or("").to_string();
    Ok(content)
}

async fn call_anthropic_for_intent(prompt: &str) -> Result<serde_json::Value, String> {
    let api_key = env::var("ANTHROPIC_API_KEY").map_err(|_| "Missing ANTHROPIC_API_KEY env var".to_string())?;
    let client = Client::new();
    let url = "https://api.anthropic.com/v1/messages";
    let system_prompt = "You are an intent extraction engine. If the user wants to send ETH, respond ONLY with this JSON: {\"action\":\"send_eth\",\"amount\":<float>,\"recipient\":\"<address>\"}. For all other requests, respond ONLY with {\"action\":\"none\"}. Do not explain, do not apologize, do not add any text. Only output valid JSON.";
    let body = serde_json::json!({
        "model": "claude-3-5-haiku-latest",
        "max_tokens": 256,
        "system": system_prompt,
        "messages": [
            {"role": "user", "content": prompt}
        ]
    });
    let resp = client.post(url)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Anthropic API error: {}", e))?;
    let json: serde_json::Value = resp.json().await.map_err(|e| format!("Anthropic JSON error: {}", e))?;
    let content = json["content"][0]["text"].as_str().unwrap_or("");
    println!("Anthropic intent response: {}", content);
    serde_json::from_str(content).map_err(|e| format!("Intent JSON parse error: {} | content: {}", e, content))
}

async fn handle_command(Json(payload): Json<CommandRequest>) -> Json<CommandResponse> {
    let command = payload.command.trim();
    // 1. Ask Anthropic for intent
    match call_anthropic_for_intent(command).await {
        Ok(intent) => {
            if intent["action"] == "send_eth" {
                // Parse amount and recipient
                let amount = intent["amount"].as_f64().unwrap_or(0.0);
                let recipient_str = intent["recipient"].as_str().unwrap_or("");
                use ethers::prelude::*;
                use std::str::FromStr;
                let recipient = match Address::from_str(recipient_str) {
                    Ok(addr) => addr,
                    Err(_) => {
                        return Json(CommandResponse {
                            response: "Invalid recipient address from intent.".to_string(),
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
            } else {
                // Not a send_eth action, fallback to Anthropic for a regular answer
                // Call Anthropic as a regular assistant
                let answer = call_anthropic(command).await.unwrap_or("(no response)".to_string());
                return Json(CommandResponse {
                    response: format!("Anthropic: {}", answer),
                });
            }
        }
        Err(e) => {
            return Json(CommandResponse {
                response: format!("Anthropic intent error: {}", e),
            });
        }
    }
}

async fn handle_reset() -> Json<ResetResponse> {
    // Send anvil_reset JSON-RPC request
    let client = Client::new();
    let url = "http://127.0.0.1:8545";
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "anvil_reset",
        "params": [],
        "id": 1
    });
    let resp = client.post(url)
        .json(&body)
        .send()
        .await;
    match resp {
        Ok(r) if r.status().is_success() => Json(ResetResponse { status: "reset ok".to_string() }),
        Ok(r) => Json(ResetResponse { status: format!("reset failed: HTTP {}", r.status()) }),
        Err(e) => Json(ResetResponse { status: format!("reset failed: {}", e) }),
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let app = Router::new()
        .route("/command", post(handle_command))
        .route("/reset", post(handle_reset));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("MCP server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
