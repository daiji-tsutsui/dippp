use crate::domain::model::i_user_context::{IUserContext, Role};

pub struct UserContextAdapter {}

impl UserContextAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

impl IUserContext for UserContextAdapter {
    fn is_in_role(&self, role: Role) -> bool {
        // Checking login session
        match role {
            Role::PreferredCustomer => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_in_role() {
        let user_context = UserContextAdapter::new();
        assert!(user_context.is_in_role(Role::PreferredCustomer));
    }
}
