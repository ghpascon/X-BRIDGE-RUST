use super::path::CONFIG_PATH;
use serde::{Deserialize, Serialize};
use std::{fs, net::SocketAddr, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfigData {
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub open_browser: bool,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub data: AppConfigData,
    path: String,
}

impl Default for AppConfigData {
    fn default() -> Self {
        Self {
            port: 3000,
            open_browser: true,
        }
    }
}

impl AppConfig {
    /// Cria e carrega config do path informado. Se não existir, cria com valores padrão.
    pub fn from_path<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path_ref = path.as_ref();
        let path_str = path_ref.to_string_lossy().to_string();
        let data = if path_ref.exists() {
            let content = fs::read_to_string(path_ref)?;
            serde_json::from_str(&content)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?
        } else {
            if let Some(parent) = path_ref.parent() {
                fs::create_dir_all(parent)?;
            }
            let data = AppConfigData::default();
            let content = serde_json::to_string_pretty(&data)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            fs::write(path_ref, content)?;
            data
        };
        Ok(AppConfig {
            data,
            path: path_str,
        })
    }

    /// Cria config usando o caminho padrão.
    pub fn from_default_path() -> std::io::Result<Self> {
        Self::from_path(CONFIG_PATH)
    }

    /// Salva a configuração atual no path associado.
    pub fn save(&self) -> std::io::Result<()> {
        if let Some(parent) = Path::new(&self.path).parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(&self.data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(&self.path, content)?;
        Ok(())
    }

    /// Retorna o path associado.
    pub fn path(&self) -> &str {
        &self.path
    }

    #[must_use]
    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::from(([0, 0, 0, 0], self.data.port))
    }

    /// Acesso direto aos dados mutáveis.
    pub fn data_mut(&mut self) -> &mut AppConfigData {
        &mut self.data
    }

    /// Acesso somente leitura aos dados.
    pub fn data(&self) -> &AppConfigData {
        &self.data
    }
}
