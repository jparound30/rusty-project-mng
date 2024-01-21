#[derive(Debug)]
pub struct UserAuthentication {
    pub password_hash: String,
    pub salt: String,
}
