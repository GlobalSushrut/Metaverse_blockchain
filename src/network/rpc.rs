use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct RPCRequest {
    pub method: String,
    pub params: serde_json::Value,
    pub id: u64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RPCResponse {
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub id: u64
}

pub struct RPCServer {
    pub port: u16
}

impl RPCServer {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation will go here
        Ok(())
    }
}
