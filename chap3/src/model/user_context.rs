pub enum Role {
    PreferredCustomer,
    NormalCustomer,
}

pub trait IUserContext {
    fn is_in_role(&self, role: Role) -> bool;
}

pub struct UserContextAdapter {}

impl UserContextAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

impl IUserContext for UserContextAdapter {
    fn is_in_role(&self, role: Role) -> bool {
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
