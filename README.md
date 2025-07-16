# AI Agent System for Ethereum Blockchain

## Overview
This project is a Rust-based AI agent system that allows users to interact with a forked Ethereum blockchain using natural language commands. The system consists of two main components:

- **AI Agent Client**: A CLI REPL built with Rust, which takes user input and sends commands to the server.
- **MCP Server**: A Rust server that parses commands, interacts with the Ethereum blockchain (via Anvil and ethers), and returns results.

---

## Features
- Parse natural language commands (e.g., `send 1 ETH from my address to 0x...`).
- Default sender is Anvil account 0.
- Validate recipient Ethereum addresses.
- Generate and send transactions on a forked Ethereum network (Anvil).
- Return transaction hash to the user.

---

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (via rustup)
- [Foundry (Anvil)](https://book.getfoundry.sh/getting-started/installation)
- [Alchemy](https://alchemy.com/) account and API key (for mainnet forking)
- [Node.js](https://nodejs.org/) (optional, for Foundry tools)

---

## Setup Instructions

### 1. Clone the Repository
```
git clone <your-repo-url>
cd RustProject
```

### 2. Install Rust Dependencies
```
cargo install --path ai_agent_client
cargo install --path mcp_server
```
Or build each project individually:
```
cargo build --release --manifest-path ai_agent_client/Cargo.toml
cargo build --release --manifest-path mcp_server/Cargo.toml
```

### 3. Install Foundry (if not already installed)
```
curl -L https://foundry.paradigm.xyz | bash
source ~/.zshenv  # or open a new terminal
foundryup
```

### 4. Start Anvil (Ethereum Fork)
```
anvil --fork-url https://eth-mainnet.g.alchemy.com/v2/<your-alchemy-key>
```
Leave this running. It will provide test accounts and private keys.

### 5. Start the MCP Server
```
cd mcp_server
cargo run
```
You should see:
```
MCP server running at http://127.0.0.1:3000
```

### 6. Start the AI Agent Client
```
cd ../ai_agent_client
cargo run
```
You should see a prompt:
```
You:
```

---

## Usage Example
At the client prompt, type:
```
send 1 ETH from my address to 0x70997970C51812dc3A010C7d01b50e0d17dc79C8
```
- This will send 1 ETH from account 0 (the default sender) to account 1.
- The client will display the transaction hash returned by the server.

---

## Project Structure
```
RustProject/
  ai_agent_client/   # CLI client
  mcp_server/        # Server handling commands and blockchain interaction
```

---

## Available Anvil Test Accounts

| Index | Address                                      | Private Key                                                        |
|-------|----------------------------------------------|--------------------------------------------------------------------|
| 0     | 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266   | 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 |
| 1     | 0x70997970C51812dc3A010C7d01b50e0d17dc79C8   | 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d |
| ...   | ...                                          | ...                                                                |

---

## Extending the System
- Add more natural language command support.
- Integrate Anthropic or other LLM APIs for advanced parsing.
- Add support for specifying sender account.
- Add transaction status queries and balance checks.

---

## Troubleshooting
- Ensure Anvil is running and accessible at `127.0.0.1:8545`.
- Use valid Ethereum addresses for recipient.
- If you see `Transaction dropped or not found.`, try again or check Anvil logs.

---

## License
MIT # RustProject
