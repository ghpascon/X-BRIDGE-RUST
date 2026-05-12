use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Response returned by the health endpoint.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: &'static str,
}
