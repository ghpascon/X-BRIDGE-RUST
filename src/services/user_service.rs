#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
}

pub struct UserService;

impl UserService {
    #[must_use]
    pub fn list() -> Vec<User> {
        vec![User {
            id: 1,
            name: "Admin".to_string(),
        }]
    }

    #[must_use]
    pub fn find_by_id(id: u64) -> Option<User> {
        Self::list().into_iter().find(|u| u.id == id)
    }
}
