use crate::shorty::form::add_edit_url_form::AddEditUrlValidated;
use crate::shorty::model::shorty_model::{GetUrlRedirectModel, GetUserIdByUrlIdModel};
use crate::shorty::repository::shorty_repository::ShortyRepository;
use error_stack::{Report, ResultExt};
use poem::http::StatusCode;
use shared::context::{Context, ContextError, FromContext};

#[derive(Debug, thiserror::Error)]
pub enum EditUrlServiceError {
    #[error("Database error")]
    DbError,
}

pub struct EditUrlService {
    shorty_repository: ShortyRepository,
}

impl EditUrlService {
    pub fn new(shorty_repository: ShortyRepository) -> Self {
        Self { shorty_repository }
    }

    pub fn get_url_redirect(
        &self,
        id: i64,
    ) -> Result<GetUrlRedirectModel, Report<EditUrlServiceError>> {
        self.shorty_repository
            .get_url_redirect(id)
            .change_context(EditUrlServiceError::DbError)?
            .ok_or_else(|| Report::new(EditUrlServiceError::DbError).attach(StatusCode::NOT_FOUND))
    }

    pub fn edit_url_submit(
        &self,
        form: &AddEditUrlValidated,
        id: i64,
    ) -> Result<(), Report<EditUrlServiceError>> {
        self.shorty_repository
            .edit_url_redirect(id, form.url_path.as_str(), form.url_redirect.as_str())
            .change_context(EditUrlServiceError::DbError)?;

        Ok(())
    }

    pub fn fetch_user_id_from_url_id(
        &self,
        id: i64,
    ) -> Result<GetUserIdByUrlIdModel, Report<EditUrlServiceError>> {
        self.shorty_repository
            .get_user_id_by_url_id(id)
            .change_context(EditUrlServiceError::DbError)?
            .ok_or_else(|| Report::new(EditUrlServiceError::DbError).attach(StatusCode::NOT_FOUND))
    }
}

impl FromContext for EditUrlService {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        Ok(Self::new(ctx.inject().await?))
    }
}
