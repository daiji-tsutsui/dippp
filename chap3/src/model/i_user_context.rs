pub trait IUserContext {
    fn is_in_role(&self, role: &str) -> bool;
}
