use crate::shorty::model::shorty_model::GetUserIdByUrlIdModel;
use crate::shorty::repository::shorty_repository::ShortyRepository;
use error_stack::{Report, ResultExt};
use poem::http::StatusCode;
use shared::context::{Context, ContextError, FromContext};

#[derive(Debug, thiserror::Error)]
pub enum DeleteUrlServiceError {
    #[error("Database error")]
    DbError,
}

pub struct DeleteUrlService {
    shorty_repository: ShortyRepository,
}

impl DeleteUrlService {
    pub fn new(shorty_repository: ShortyRepository) -> Self {
        Self { shorty_repository }
    }

    pub fn delete_url(&self, id: i64) -> Result<(), Report<DeleteUrlServiceError>> {
        self.shorty_repository
            .delete_url_redirect(id)
            .change_context(DeleteUrlServiceError::DbError)?;

        Ok(())
    }

    pub fn fetch_user_id_from_url_id(
        &self,
        id: i64,
    ) -> Result<GetUserIdByUrlIdModel, Report<DeleteUrlServiceError>> {
        self.shorty_repository
            .get_user_id_by_url_id(id)
            .change_context(DeleteUrlServiceError::DbError)?
            .ok_or_else(|| {
                Report::new(DeleteUrlServiceError::DbError).attach(StatusCode::NOT_FOUND)
            })
    }
}

impl FromContext for DeleteUrlService {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        Ok(Self::new(ctx.inject().await?))
    }
}
