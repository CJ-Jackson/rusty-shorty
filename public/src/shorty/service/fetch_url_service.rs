use crate::shorty::model::url::UrlRedirect;
use crate::shorty::repository::shorty::ShortyRepository;
use error_stack::{Report, ResultExt};
use poem::http::StatusCode;
use shared::context::{Context, ContextError, FromContext};
use shared::error::ExtraResultExt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FetchUrlServiceError {
    #[error("Db error")]
    DbError,
    #[error("Not Found")]
    NotFound,
}

pub struct FetchUrlService {
    shorty_repository: ShortyRepository,
}

impl FetchUrlService {
    pub fn new(shorty_repository: ShortyRepository) -> Self {
        Self { shorty_repository }
    }

    pub fn fetch_url(&self, path: &str) -> Result<UrlRedirect, Report<FetchUrlServiceError>> {
        let url_redirect = self
            .shorty_repository
            .fetch_url(path)
            .change_context(FetchUrlServiceError::DbError)
            .log_it()?
            .ok_or_else(|| {
                Report::new(FetchUrlServiceError::NotFound)
                    .attach(format!("Path: {}", path))
                    .attach(StatusCode::NOT_FOUND)
            })?;
        Ok(url_redirect)
    }
}

impl FromContext for FetchUrlService {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        Ok(Self::new(ctx.inject().await?))
    }
}
