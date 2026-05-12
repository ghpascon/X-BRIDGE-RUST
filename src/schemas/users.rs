use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Compact user representation returned in list responses.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserSummary {
    pub id: u64,
    pub name: String,
}

/// Payload accepted when creating a new user.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub name: String,
}

/// Full user representation returned after create/get operations.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: u64,
    pub name: String,
}
