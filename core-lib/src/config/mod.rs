use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubnetraConfig {
    pub mode: String,
    pub listen_port: u16,
    pub tun: String,
    pub local_id: u32,
    pub public_key: String,
    pub private_key: String,
    pub preshared_key: Option<String>,
    #[serde(default)]
    pub peers: Vec<PeerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerConfig {
    pub id: u32,
    pub name: Option<String>,
    pub endpoint: String,
    pub allowed_src: String,
    pub public_key: String,
}

impl SubnetraConfig {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: SubnetraConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}