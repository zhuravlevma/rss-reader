#[derive(Clone)]
pub enum AuthState {
    Auth,
    UnAuth,
}

impl Default for AuthState {
    fn default() -> Self {
        AuthState::UnAuth
    }
}

#[derive(Clone, Default)]
pub struct UserStore {
    pub auth_state: AuthState,
    pub token: String,
    pub user_id: String,
}