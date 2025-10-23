/// Module MCP - Model Context Protocol (serveur + outils)

pub mod server;
pub mod protocol;
pub mod tools;

pub use server::MCPServer;
pub use protocol::{JsonRpcRequest, JsonRpcResponse, ServerInfo};
pub use tools::{Tool, ToolHandler, ToolRegistry};
