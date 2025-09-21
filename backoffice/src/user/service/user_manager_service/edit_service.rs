use crate::user::form::edit_user::EditUserValidated;
use crate::user::model::user_manager_model::FetchUser;
use crate::user::repository::user_manager_repository::UserManagerRepository;
use cjtoolkit_structured_validator::types::username::IsUsernameTakenAsync;
use error_stack::{Report, ResultExt};
use poem::http::StatusCode;
use shared::context::{Context, ContextError, FromContext};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EditUserServiceError {
    #[error("User already exists")]
    SubmitFailed,
    #[error("User not found")]
    UserNotFound,
}

pub struct EditUserService {
    user_manager_repository: UserManagerRepository,
}

impl EditUserService {
    pub fn new(user_manager_repository: UserManagerRepository) -> Self {
        Self {
            user_manager_repository,
        }
    }

    pub fn edit_user_submit(
        &self,
        user_id: i64,
        edit_user_validated: &EditUserValidated,
    ) -> Result<(), Report<EditUserServiceError>> {
        self.user_manager_repository
            .edit_user(
                user_id,
                edit_user_validated.username.as_str().to_string(),
                &edit_user_validated.role,
            )
            .change_context(EditUserServiceError::SubmitFailed)?;
        Ok(())
    }

    pub fn fetch_user(&self, user_id: i64) -> Result<FetchUser, Report<EditUserServiceError>> {
        self.user_manager_repository
            .fetch_user(user_id)
            .change_context(EditUserServiceError::UserNotFound)?
            .ok_or_else(|| {
                Report::new(EditUserServiceError::UserNotFound).attach(StatusCode::NOT_FOUND)
            })
    }
}

impl IsUsernameTakenAsync for EditUserService {
    async fn is_username_taken_async(&self, username: &str) -> bool {
        self.user_manager_repository
            .username_taken(username.to_string())
            .ok()
            .unwrap_or_default()
    }
}

impl FromContext for EditUserService {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        Ok(Self::new(ctx.inject().await?))
    }
}
