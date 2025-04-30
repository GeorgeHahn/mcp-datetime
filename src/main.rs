use chrono::{DateTime, Local};
use rmcp::{
    model::{ServerCapabilities, ServerInfo},
    tool, ServerHandler, ServiceExt,
};
use tokio::io::{stdin, stdout};

#[derive(Debug, Clone)]
pub struct DateTimeServer;

// Create a static toolbox to store the tool attributes
#[tool(tool_box)]
impl DateTimeServer {
    #[tool(description = "Get the current local date and time in ISO 8601 format")]
    async fn get_current_date_time(&self) -> String {
        let local: DateTime<Local> = Local::now();

        local.to_rfc3339()
    }
}

// Implement ServerHandler by querying static toolbox
#[tool(tool_box)]
impl ServerHandler for DateTimeServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A server that provides the current date and time".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = DateTimeServer;
    let transport = (stdin(), stdout());

    // Initialize and serve
    let server = service.serve(transport).await?;

    // Wait for server shutdown
    let quit_reason = server.waiting().await?;
    println!("Server shutdown: {:?}", quit_reason);

    Ok(())
}
