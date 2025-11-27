use std::io::{self, BufRead};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct RpcRequest {
    jsonrpc: String,
    method: String,
    params: Value,
    id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct RpcResponse {
    jsonrpc: String,
    result: Option<Value>,
    error: Option<String>,
    id: u64,
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        eprintln!("SSL AI Daemon (ssld) v0.1 started.");
        eprintln!("Listening on Stdin...");

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(input) => {
                    if let Ok(req) = serde_json::from_str::<RpcRequest>(&input) {
                        eprintln!("Received request: {:?}", req);
                        
                        let result = match req.method.as_str() {
                            "ping" => Some(Value::String("pong".to_string())),
                            "explain_error" => {
                                if let Some(error_msg) = req.params.get("error").and_then(|v| v.as_str()) {
                                    match call_ollama(error_msg).await {
                                        Ok(response) => Some(Value::String(response)),
                                        Err(e) => Some(Value::String(format!("AI Error: {}", e))),
                                    }
                                } else {
                                    Some(Value::String("Missing 'error' parameter".to_string()))
                                }
                            }
                            "expand_natural" => {
                                if let Some(prompt) = req.params.get("prompt").and_then(|v| v.as_str()) {
                                    match call_ollama(prompt).await {
                                        Ok(response) => Some(Value::String(response)),
                                        Err(e) => Some(Value::String(format!("AI Error: {}", e))),
                                    }
                                } else {
                                    Some(Value::String("Missing 'prompt' parameter".to_string()))
                                }
                            }
                            _ => None,
                        };

                        let resp = RpcResponse {
                            jsonrpc: "2.0".to_string(),
                            error: if result.is_none() { Some("Method not found".to_string()) } else { None },
                            result,
                            id: req.id,
                        };

                        println!("{}", serde_json::to_string(&resp).unwrap());
                    } else {
                        eprintln!("Invalid JSON-RPC");
                    }
                }
                Err(_) => break,
            }
        }
    });
}

async fn call_ollama(prompt: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    
    let payload = serde_json::json!({
        "model": "qwen2.5-coder:1.5b-base",
        "prompt": prompt,
        "stream": false
    });

    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("HTTP Error: {}", e))?;

    let json: Value = response.json().await.map_err(|e| format!("JSON Error: {}", e))?;
    
    json.get("response")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| "No response field in Ollama response".to_string())
}
