use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();

    argon
        .hash_password(password.as_bytes(), &salt)
        .expect("Shoudl not failed")
        .to_string()
}

pub fn verify_pass(password: &str, hashed: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hashed) {
        Ok(h) => h,
        Err(_) => return false,
    };
    // ce verify est la fonction intégré de Argon2
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_correct_password() {
        let password = "motdepasse123";
        let hash = hash_password(password);

        assert!(verify_pass(password, &hash));
    }

    #[test]
    fn test_verify_wrong_password_fails() {
        let password = "motdepasse123";
        let hash = hash_password(password);

        assert!(!verify_pass("mauvais_mot_de_passe", &hash));
    }

    #[test]
    fn test_same_password_produces_different_hashes() {
        let password = "motdepasse123";
        let hash1 = hash_password(password);
        let hash2 = hash_password(password);

        assert_ne!(hash1, hash2);
    }
}
