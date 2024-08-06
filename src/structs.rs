#[derive(Debug)]
pub struct User {
    pub username: String,
    pub email: String,
    pub active: bool,
    pub sign_in_count: u32,
}
