# mcp-datetime

An MCP server that provides date and time functionality.

## Features

- Provide current date and time in ISO 8601 format

## Installation

To install mcp-datetime, you'll need to have Rust and Cargo installed on your system. Then you can install it directly from GitHub using:

```bash
cargo install --git https://github.com/GeorgeHahn/mcp-datetime
```

## MCP Setup

Add the following configuration to your MCP configuration file (typically located at `~/.cursor/mcp.json`):

```json
{
    "datetime": {
        "command": "mcp-datetime"
    }
}
```
