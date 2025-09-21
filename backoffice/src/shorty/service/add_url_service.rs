use crate::shorty::form::add_edit_url_form::AddEditUrlValidated;
use crate::shorty::repository::shorty_repository::ShortyRepository;
use error_stack::{Report, ResultExt};
use shared::context::{Context, ContextError, FromContext};

#[derive(Debug, thiserror::Error)]
pub enum AddUrlServiceError {
    #[error("Database error")]
    DbError,
}

pub struct AddUrlService {
    shorty_repository: ShortyRepository,
}

impl AddUrlService {
    pub fn new(shorty_repository: ShortyRepository) -> Self {
        Self { shorty_repository }
    }

    pub fn add_url_submit(
        &self,
        form: &AddEditUrlValidated,
        user_id: i64,
    ) -> Result<(), Report<AddUrlServiceError>> {
        self.shorty_repository
            .add_url_redirect(form.url_path.as_str(), form.url_redirect.as_str(), user_id)
            .change_context(AddUrlServiceError::DbError)?;

        Ok(())
    }
}

impl FromContext for AddUrlService {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        Ok(Self::new(ctx.inject().await?))
    }
}
