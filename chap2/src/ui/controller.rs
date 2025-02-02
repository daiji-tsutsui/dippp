pub mod home;

pub struct HttpSession {
    pub user: UserHttpContext,
}

pub struct UserHttpContext {
    pub role: String,
}

impl UserHttpContext {
    pub fn is_in_role(&self, target_role: &str) -> bool {
        self.role == String::from(target_role)
    }
}
