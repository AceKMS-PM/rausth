use crate::core::identity::User;

#[derive(Debug)]
pub enum RepositoryError {
    NotFound,
    AlreadyExists,
    Database(String),
}
pub trait UserRepository {
    async fn save(&self, user: &User) -> Result<(), RepositoryError>;
    async fn find_by_email(&self, email: &str) -> Result<User, RepositoryError>;
}
