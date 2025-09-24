use crate::error::FromErrorStack;
use error_stack::Report;
use poem::{FromRequest, Request, RequestBody};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContextError {
    #[error("Config error")]
    ConfigError,
    #[error("Request error")]
    RequestError,
    #[error("Other error")]
    Other,
}

pub trait FromContext: Sized + Send + Sync {
    fn from_context(
        ctx: &'_ Context,
    ) -> impl Future<Output = Result<Self, Report<ContextError>>> + Send;
}

pub struct Context<'a> {
    pub req: Option<&'a Request>,
}

impl Context<'_> {
    pub async fn inject<T: FromContext>(&self) -> Result<T, Report<ContextError>> {
        T::from_context(self).await
    }
}

pub struct Dep<T: FromContext>(pub T);

impl<'a, T: FromContext> FromRequest<'a> for Dep<T> {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> poem::Result<Self> {
        let context = Box::pin(Context { req: Some(req) });
        Ok(Self(
            T::from_context(&context)
                .await
                .map_err(poem::Error::from_error_stack)?,
        ))
    }
}

impl<T: FromContext> Dep<T> {
    pub async fn without_request() -> Result<T, Report<ContextError>> {
        T::from_context(&Context { req: None }).await
    }
}
