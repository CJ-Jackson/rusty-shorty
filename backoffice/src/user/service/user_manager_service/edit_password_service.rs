use crate::user::form::edit_password_manager::EditPasswordManagerValidated;
use crate::user::layer::password_layer::PasswordLayer;
use crate::user::model::user_manager_model::FetchUser;
use crate::user::repository::user_manager_repository::UserManagerRepository;
use error_stack::{Report, ResultExt};
use poem::http::StatusCode;
use shared::context::{Context, ContextError, FromContext};
use shared::error::ExtraResultExt;
use shared::password::Password;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EditPasswordServiceError {
    #[error("User not found")]
    UserNotFound,
    #[error("Database error")]
    DbError,
    #[error("Password Hash Error")]
    PasswordHashError,
    #[error("Password Serialize Error")]
    PasswordSerializeError,
}

pub struct EditPasswordService {
    user_manager_repository: UserManagerRepository,
    password_layer: PasswordLayer,
}

impl EditPasswordService {
    pub fn new(
        user_manager_repository: UserManagerRepository,
        password_layer: PasswordLayer,
    ) -> Self {
        Self {
            user_manager_repository,
            password_layer,
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
                    .change_context(EditPasswordServiceError::PasswordSerializeError)
                    .log_it()
                    .attach(StatusCode::INTERNAL_SERVER_ERROR)?,
            )
            .change_context(EditPasswordServiceError::DbError)?;

        Ok(())
    }

    pub fn fetch_user(&self, user_id: i64) -> Result<FetchUser, Report<EditPasswordServiceError>> {
        self.user_manager_repository
            .fetch_user(user_id)
            .change_context(EditPasswordServiceError::UserNotFound)?
            .ok_or_else(|| {
                Report::new(EditPasswordServiceError::UserNotFound).attach(StatusCode::NOT_FOUND)
            })
    }

    fn hash_password(&self, password: &str) -> Result<Password, Report<EditPasswordServiceError>> {
        self.password_layer
            .hash_password(password)
            .change_context(EditPasswordServiceError::PasswordHashError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl FromContext for EditPasswordService {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        Ok(Self::new(ctx.inject().await?, ctx.inject().await?))
    }
}
