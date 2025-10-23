/// MCP (Model Context Protocol) Server

use super::protocol::*;
use super::tools::ToolRegistry;
use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};

/// Shared state of the MCP server
pub struct MCPServerState {
    tool_registry: Arc<RwLock<ToolRegistry>>,
    server_info: ServerInfo,
}

/// Main MCP server
pub struct MCPServer {
    state: Arc<MCPServerState>,
    port: u16,
}

impl MCPServer {
    /// Creates a new instance of the MCP server
    pub fn new(port: u16) -> Self {
        info!("Initializing MCP server on port {}", port);
        
        let server_info = ServerInfo {
            name: "agents-rs".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            protocol_version: MCP_VERSION.to_string(),
            capabilities: ServerCapabilities::default(),
        };

        let state = Arc::new(MCPServerState {
            tool_registry: Arc::new(RwLock::new(ToolRegistry::new())),
            server_info,
        });

        Self { state, port }
    }

    /// Starts the MCP server
    pub async fn start(&self) -> Result<()> {
        let app = Router::new()
            .route("/", get(health_check))
            .route("/mcp", post(handle_mcp_request))
            .with_state(Arc::clone(&self.state));

        let addr = format!("127.0.0.1:{}", self.port);
        info!("MCP server listening on http://{}", addr);
        
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }

    /// Returns the tool registry
    pub fn tool_registry(&self) -> Arc<RwLock<ToolRegistry>> {
        Arc::clone(&self.state.tool_registry)
    }
}

/// Handler for health check
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "agents-rs MCP Server"
    }))
}

/// Main handler for MCP requests
async fn handle_mcp_request(
    State(state): State<Arc<MCPServerState>>,
    Json(request): Json<JsonRpcRequest>,
) -> impl IntoResponse {
    info!("MCP request received: {}", request.method);

    let response = match request.method.as_str() {
        "initialize" => handle_initialize(&state, request).await,
        "tools/list" => handle_list_tools(&state, request).await,
        "tools/call" => handle_call_tool(&state, request).await,
        _ => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code: -32601,
                message: format!("Method not found: {}", request.method),
                data: None,
            }),
            id: request.id,
        },
    };

    (StatusCode::OK, Json(response))
}

/// Handles initialization request
async fn handle_initialize(
    state: &MCPServerState,
    request: JsonRpcRequest,
) -> JsonRpcResponse {
    info!("Initializing MCP server");

    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        result: Some(serde_json::to_value(&state.server_info).unwrap()),
        error: None,
        id: request.id,
    }
}

/// Handles list tools request
async fn handle_list_tools(
    state: &MCPServerState,
    request: JsonRpcRequest,
) -> JsonRpcResponse {
    let registry = state.tool_registry.read().await;
    let tools = registry.list_tools();

    let tools_desc: Vec<ToolDescription> = tools
        .iter()
        .map(|tool| ToolDescription {
            name: tool.name.clone(),
            description: tool.description.clone(),
            input_schema: tool.input_schema.clone(),
        })
        .collect();

    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        result: Some(serde_json::json!({ "tools": tools_desc })),
        error: None,
        id: request.id,
    }
}

/// Handles tool call request
async fn handle_call_tool(
    state: &MCPServerState,
    request: JsonRpcRequest,
) -> JsonRpcResponse {
    let params: CallToolParams = match request.params {
        Some(ref p) => match serde_json::from_value(p.clone()) {
            Ok(params) => params,
            Err(e) => {
                error!("Parameter parsing error: {}", e);
                return JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: "Invalid parameters".to_string(),
                        data: Some(serde_json::json!({ "error": e.to_string() })),
                    }),
                    id: request.id,
                };
            }
        },
        None => {
            return JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32602,
                    message: "Missing parameters".to_string(),
                    data: None,
                }),
                id: request.id,
            };
        }
    };

    let registry = state.tool_registry.read().await;
    
    match registry.execute_tool(&params.name, params.arguments).await {
        Ok(result) => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": result
                }]
            })),
            error: None,
            id: request.id,
        },
        Err(e) => {
            error!("Tool execution error for {}: {}", params.name, e);
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32000,
                    message: format!("Tool execution error: {}", e),
                    data: None,
                }),
                id: request.id,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let server = MCPServer::new(3000);
        assert_eq!(server.port, 3000);
    }

    #[test]
    fn test_json_rpc_request() {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "initialize".to_string(),
            params: None,
            id: Some(serde_json::json!(1)),
        };
        assert_eq!(request.method, "initialize");
    }
}
