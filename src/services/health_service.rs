#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub status: &'static str,
}

pub struct HealthService;

impl HealthService {
    #[must_use]
    pub fn check() -> HealthStatus {
        HealthStatus { status: "ok" }
    }
}
