use std::process::Command;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Web2AppConfig {
    pub app_id: String,
    pub docker_image: String,
    pub command: Vec<String>,
    pub env_vars: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Web2AppResult {
    pub app_id: String,
    pub output: Vec<u8>,
    pub timestamp: u64,
    pub proof: [u8; 32],
}

pub struct Web2Runner {
    proofs: HashMap<String, Web2AppResult>,
}

impl Web2Runner {
    pub fn new() -> Self {
        Self {
            proofs: HashMap::new(),
        }
    }

    pub fn run_app(&mut self, config: Web2AppConfig) -> Result<Web2AppResult, String> {
        // Run Docker container
        let mut cmd = Command::new("docker");
        cmd.arg("run")
           .arg("--rm")
           .arg(&config.docker_image);
        
        // Add environment variables
        for (key, value) in &config.env_vars {
            cmd.arg("-e").arg(format!("{}={}", key, value));
        }
        
        // Add command
        cmd.args(&config.command);

        let output = cmd.output()
            .map_err(|e| format!("Failed to run docker container: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into_owned());
        }

        // Generate proof using Blake3
        let mut hasher = blake3::Hasher::new();
        hasher.update(&output.stdout);
        hasher.update(&output.stderr);
        let proof = *hasher.finalize().as_bytes();

        // Create result
        let result = Web2AppResult {
            app_id: config.app_id.clone(),
            output: output.stdout,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            proof,
        };

        // Store proof
        self.proofs.insert(config.app_id, result.clone());

        Ok(result)
    }

    pub fn get_proof(&self, app_id: &str) -> Option<&Web2AppResult> {
        self.proofs.get(app_id)
    }

    pub fn get_all_proofs(&self) -> Vec<&Web2AppResult> {
        self.proofs.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web2_runner() {
        let mut runner = Web2Runner::new();
        let config = Web2AppConfig {
            app_id: "test-python".to_string(),
            docker_image: "python:3.9-slim".to_string(),
            command: vec!["python".to_string(), "-c".to_string(), "print('hello')".to_string()],
            env_vars: HashMap::new(),
        };

        let result = runner.run_app(config);
        assert!(result.is_ok());
    }
}
