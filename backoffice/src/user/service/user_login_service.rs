use crate::user::LOGIN_TOKEN_COOKIE_NAME;
use crate::user::repository::user_repository::UserRepository;
use error_stack::Report;
use shared::context::{Context, ContextError, FromContext};
use shared::password::Password;
use uuid::Uuid;

pub struct UserLoginService {
    user_repository: UserRepository,
    token_cookie: Option<String>,
}

impl UserLoginService {
    pub fn new(user_repository: UserRepository, token_cookie: Option<String>) -> Self {
        Self {
            user_repository,
            token_cookie,
        }
    }

    pub fn validate_login(&self, username: String, password: String) -> Option<String> {
        if let Ok(is_password) = self.user_repository.get_user_password(username) {
            let password_status = Password::verify_password(is_password.password, password);
            if let Ok(password_state) = password_status {
                if password_state.is_valid() {
                    let uuid = Uuid::new_v4().to_string();

                    if self
                        .user_repository
                        .add_token(uuid.clone(), is_password.id)
                        .is_err()
                    {
                        return None;
                    }

                    return Some(uuid);
                }
            }
        }
        None
    }

    pub fn logout(&self) -> bool {
        if let Some(token) = self.token_cookie.as_ref() {
            self.user_repository.delete_token(token.to_string()).is_ok()
        } else {
            false
        }
    }
}

impl FromContext for UserLoginService {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        let req = ctx
            .req
            .ok_or_else(|| Report::new(ContextError::RequestError))?;
        let cookie = req.cookie();
        Ok(Self::new(
            ctx.inject().await?,
            cookie
                .get(LOGIN_TOKEN_COOKIE_NAME)
                .map(|v| v.value_str().to_string()),
        ))
    }
}
