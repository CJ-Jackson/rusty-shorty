use std::ops::{Deref, DerefMut};
use poem::{FromRequest, Request, RequestBody};
use poem::error::ResponseError;
use poem::http::StatusCode;
use serde::de::DeserializeOwned;

/// A possible error value when parsing query.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct ParseQueryError(#[from] pub serde_qs::Error);

impl ResponseError for ParseQueryError {
    fn status(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct QueryQs<T>(pub T);

impl<T> Deref for QueryQs<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for QueryQs<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: DeserializeOwned> QueryQs<T> {
    async fn internal_from_request(req: &Request) -> Result<Self, ParseQueryError> {
        Ok(serde_qs::from_str(req.uri().query().unwrap_or_default()).map(Self)?)
    }
}

impl<'a, T: DeserializeOwned> FromRequest<'a> for QueryQs<T> {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> poem::Result<Self>  {
        Self::internal_from_request(req).await.map_err(Into::into)
    }
}
