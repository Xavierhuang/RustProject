# AI Agent System for Ethereum Blockchain

## Overview
This project is a Rust-based AI agent system that allows users to interact with a forked Ethereum blockchain using natural language commands. The system consists of two main components:

- **AI Agent Client**: A CLI REPL built with the [RIG framework](https://github.com/0xPlaygrounds/rig), which takes user input, uses Anthropic (Claude) for LLM intent extraction, and calls blockchain tools on the backend via the MCP protocol.
- **MCP Server**: A Rust server built with the [RMCP (MCP Rust SDK)](https://github.com/modelcontextprotocol/rust-sdk), exposing Foundry/Anvil blockchain functionality as tool calls, and using the [Anthropic Rust SDK](https://crates.io/crates/anthropic_sdk) for LLM intent extraction.

---

## Architecture

```
┌─────────────────┐    MCP Protocol    ┌──────────────────┐
│   RIG Agent     │◄──────────────────►│   MCP Server     │
│   (Client)      │                    │                  │
├─────────────────┤                    ├──────────────────┤
│ • CLI REPL      │                    │ • Foundry - Cast │
│ • LLM API Key   │                    │ • Tx Generation  │
│ • User Input    │                    │ • State Fork     │
│ • Response      │                    │ • Anthropic SDK  │
└─────────────────┘                    └──────────────────┘
         │                                       │
         │                                       │
         └───────────────┐           ┌───────────┘
                         │           │
                    ┌────▼───────────▼──────┐
                    │   Forked Ethereum     │
                    │     Test Network      │
                    │   (via Foundry)       │
                    └───────────────────────┘
```

---

## Features
- Natural language command parsing and intent extraction (Anthropic Claude)
- RIG-based CLI agent for user interaction
- MCP protocol for client-server communication
- Blockchain tool calls: send ETH, get balance, and more
- Real transaction generation and state management via Foundry/Anvil
- Secure handling of secrets and build artifacts

---

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (via rustup)
- [Foundry (Anvil)](https://book.getfoundry.sh/getting-started/installation)
- [Anthropic API Key](https://docs.anthropic.com/en/api/overview)
- (Optional) [Alchemy](https://alchemy.com/) account and API key (for mainnet forking)

---

## Setup Instructions

### 1. Clone the Repository
```
git clone <your-repo-url>
cd RustProject
```

### 2. Install Rust Dependencies
```
cargo build --workspace
```

### 3. Install Foundry (if not already installed)
```
curl -L https://foundry.paradigm.xyz | bash
foundryup
```

### 4. Start Anvil (Ethereum Fork)
```
anvil --fork-url https://eth-mainnet.g.alchemy.com/v2/<your-alchemy-key>
```
Leave this running. It will provide test accounts and private keys.

### 5. Set Up Environment Variables
Create a `.env` file in both `mcp_server/` and the project root with your Anthropic API key:
```
ANTHROPIC_API_KEY=your_api_key_here
```

### 6. Start the MCP Server
```
cargo run -p mcp_server
```
You should see:
```
MCP server running at http://127.0.0.1:3000
```

### 7. Start the AI Agent Client
```
cargo run -p ai_agent_client
```
You should see a prompt:
```
You:
```

---

## Usage Example
At the client prompt, type:
```
send 1 ETH to 0x70997970C51812dc3A010C7d01b50e0d17dc79C8
```
- The client will use Anthropic to extract intent and call the `send_eth` tool on the server.
- The server will send 1 ETH from the default Anvil account to the recipient and return the transaction hash.

You can also query balances:
```
balance 0x70997970C51812dc3A010C7d01b50e0d17dc79C8
```

---

## Project Structure
```
RustProject/
  ai_agent_client/   # CLI client (RIG-based)
  mcp_server/        # Server handling commands and blockchain interaction
```

---

## Best Practices
- **Never commit secrets**: `.env` files are in `.gitignore` and should never be tracked.
- **Never commit build artifacts**: `target/` is in `.gitignore` and should never be tracked.
- **Regenerate API keys** if they were ever committed.
- **Use separate environments** for development and production.

---

## Extending the System
- Add more tool calls (e.g., reset, snapshot, advanced queries)
- Improve error handling and user experience
- Write integration tests and documentation for production-readiness

---

## License
MIT
