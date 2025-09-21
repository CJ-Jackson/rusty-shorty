use crate::user::form::add_user::AddUserValidated;
use crate::user::repository::user_manager_repository::UserManagerRepository;
use cjtoolkit_structured_validator::types::username::IsUsernameTakenAsync;
use error_stack::{Report, ResultExt};
use poem::http::StatusCode;
use shared::context::{Context, ContextError, FromContext};
use shared::password::Password;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AddUserServiceError {
    #[error("User already exists")]
    SubmitFailed,
    #[error("Password Hash Error")]
    PasswordHashError,
    #[error("Password Serialize Error")]
    PasswordSerializeError,
}

pub struct AddUserService {
    user_manager_repository: UserManagerRepository,
}

impl AddUserService {
    pub fn new(user_manager_repository: UserManagerRepository) -> Self {
        Self {
            user_manager_repository,
        }
    }

    pub fn add_user_submit(
        &self,
        add_user_validated: &AddUserValidated,
    ) -> Result<(), Report<AddUserServiceError>> {
        self.user_manager_repository
            .add_user(
                add_user_validated.username.as_str().to_string(),
                self.hash_password(add_user_validated.password.as_str())?
                    .encode_to_msg_pack()
                    .change_context(AddUserServiceError::PasswordSerializeError)?,
                &add_user_validated.role,
            )
            .change_context(AddUserServiceError::SubmitFailed)?;
        Ok(())
    }

    fn hash_password(&self, password: &str) -> Result<Password, Report<AddUserServiceError>> {
        Password::hash_password(password.to_string())
            .change_context(AddUserServiceError::PasswordHashError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl IsUsernameTakenAsync for AddUserService {
    async fn is_username_taken_async(&self, username: &str) -> bool {
        self.user_manager_repository
            .username_taken(username.to_string())
            .ok()
            .unwrap_or_default()
    }
}

impl FromContext for AddUserService {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        Ok(Self::new(ctx.inject().await?))
    }
}
