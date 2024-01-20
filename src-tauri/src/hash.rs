pub mod hash {
    use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};

    use argon2::Argon2;
    use password_hash::rand_core::OsRng;

    pub fn hash(password: String) -> (String, String)  {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        // Hash password to PHC string ($argon2id$v=19$...)
        let hashed_pw = argon2.hash_password(password.as_bytes(), &salt).expect("failed to create hash").to_string();
        (hashed_pw, salt.to_string())
    }

    pub fn verify(input_password: String, hash_string: String) -> bool {
        let password_hash = PasswordHash::new(&hash_string).expect("invalid password hash");

        // Trait objects for algorithms to support
        let algorithms: &[&dyn PasswordVerifier] = &[&Argon2::default()];

        let verify_result = password_hash.verify_password(algorithms, input_password);
        verify_result.is_ok()
    }
}