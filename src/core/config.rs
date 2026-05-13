use super::path::CONFIG_PATH;
use serde::{Deserialize, Serialize};
use std::{fs, net::SocketAddr, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub port: u16,
}

impl AppConfig {
    #[must_use]
    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::from(([0, 0, 0, 0], self.port))
    }

    /// Carrega configuração de um arquivo JSON do caminho informado. Se não existir, cria com valores padrão.
    pub fn from_path<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path = path.as_ref();
        if path.exists() {
            let data = fs::read_to_string(path)?;
            let config: AppConfig = serde_json::from_str(&data)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            Ok(config)
        } else {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let config = AppConfig::default();
            let data = serde_json::to_string_pretty(&config)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            fs::write(path, data)?;
            Ok(config)
        }
    }

    /// Carrega configuração do caminho padrão (CONFIG_PATH).
    pub fn from_default_path() -> std::io::Result<Self> {
        Self::from_path(CONFIG_PATH)
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self { port: 3000 }
    }
}
