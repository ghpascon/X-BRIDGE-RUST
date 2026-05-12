use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: [u8; 4],
    pub port: u16,
}

impl AppConfig {
    #[must_use]
    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::from((self.host, self.port))
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            host: [127, 0, 0, 1],
            port: 3000,
        }
    }
}
