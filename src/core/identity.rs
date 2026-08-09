use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hashed: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    fn new(email: String, password_hashed: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            password_hashed,
            created_at: Utc::now(),
        }
    }
}
