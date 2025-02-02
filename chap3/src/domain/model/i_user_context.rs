pub enum Role {
    PreferredCustomer,
    #[allow(dead_code)]
    NormalCustomer,
}

pub trait IUserContext {
    fn is_in_role(&self, role: Role) -> bool;
}
