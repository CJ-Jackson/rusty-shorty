use crate::user::form::edit_password_manager::EditPasswordManagerValidated;
use crate::user::repository::user_manager_repository::UserManagerRepository;
use crate::user::service::user_manager_service::add_user_service::AddUserServiceError;
use error_stack::{Report, ResultExt};
use shared::context::{Context, ContextError, FromContext};
use shared::password::Password;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EditPasswordServiceError {
    #[error("Old Password is incorrect")]
    InvalidCurrentPassword,
    #[error("User not found")]
    UserNotFound,
    #[error("Database error")]
    DbError,
    #[error("Password Hash Error")]
    PasswordHashError,
    #[error("Password Serialize Error")]
    PasswordSerializeError,
}

impl EditPasswordServiceError {
    pub fn is_invalid_current_password(&self) -> bool {
        matches!(self, EditPasswordServiceError::InvalidCurrentPassword)
    }
}

pub struct EditPasswordService {
    user_manager_repository: UserManagerRepository,
}

impl EditPasswordService {
    pub fn new(user_manager_repository: UserManagerRepository) -> Self {
        Self {
            user_manager_repository,
        }
    }

    pub fn edit_password_submit(
        &self,
        user_id: i64,
        password: &EditPasswordManagerValidated,
    ) -> Result<(), Report<EditPasswordServiceError>> {
        self.user_manager_repository
            .edit_password(
                user_id,
                self.hash_password(password.password.as_str())?
                    .encode_to_msg_pack()
                    .change_context(EditPasswordServiceError::PasswordSerializeError)?,
            )
            .change_context(EditPasswordServiceError::DbError)?;

        Ok(())
    }

    fn hash_password(&self, password: &str) -> Result<Password, Report<EditPasswordServiceError>> {
        Password::hash_password(password.to_string())
            .change_context(EditPasswordServiceError::PasswordHashError)
    }
}

impl FromContext for EditPasswordService {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        Ok(Self::new(ctx.inject().await?))
    }
}
