use rig::completion::Prompt;
use rig::providers::anthropic;
use rmcp::{
    model::CallToolRequestParam,
    service::ServiceExt,
    transport::streamable_http_client::StreamableHttpClientTransport,
};
use dialoguer::Input;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up Anthropic client (ensure ANTHROPIC_API_KEY is set in your env)
    let anthropic_client = anthropic::Client::from_env();
    let claude = anthropic_client.agent("claude-3-5-haiku-latest").build();

    // Connect to the MCP server via HTTP
    let addr = "http://127.0.0.1:3000";
    let transport = StreamableHttpClientTransport::new(addr)?;
    let service = ().serve(transport).await?;

    // List available tools
    let tools = service.list_tools(Default::default()).await?;
    println!("Available tools: {tools:#?}");

    loop {
        let input: String = Input::new().with_prompt("You").interact_text().unwrap();
        if input.trim() == "exit" { break; }

        // Use Anthropic to extract intent as JSON
        let system_prompt = "You are an intent extraction engine. If the user wants to send ETH, respond ONLY with this JSON: {\"action\":\"send_eth\",\"amount\":<float>,\"recipient\":\"<address>\"}. For all other requests, respond ONLY with {\"action\":\"none\"}. Do not explain, do not apologize, do not add any text. Only output valid JSON.";
        let anthropic_response = claude
            .with_system(system_prompt)
            .prompt(&input)
            .await;

        match anthropic_response {
            Ok(resp) => {
                // Try to parse as JSON intent
                if let Ok(intent): Result<Value, _> = serde_json::from_str(&resp) {
                    if intent["action"] == "send_eth" {
                        let amount = intent["amount"].as_f64().unwrap_or(0.0);
                        let recipient = intent["recipient"].as_str().unwrap_or("").to_string();
                        let result = service
                            .call_tool(CallToolRequestParam {
                                name: "send_eth".into(),
                                arguments: Some(serde_json::json!({
                                    "amount": amount,
                                    "recipient": recipient,
                                })),
                            })
                            .await?;
                        println!("Result: {result:#?}");
                        continue;
                    }
                }
                // Fallback: print LLM response
                println!("Agent: {}", resp);
            }
            Err(e) => println!("Error: {}", e),
        }
    }
    Ok(())
}
