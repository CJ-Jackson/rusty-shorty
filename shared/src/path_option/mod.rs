use poem::web::Path;
use poem::{FromRequest, Request, RequestBody};
use serde::de::DeserializeOwned;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PathOption<T: Default + DeserializeOwned>(pub T);

impl<T: Default + DeserializeOwned> Deref for PathOption<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Default + DeserializeOwned> DerefMut for PathOption<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T: Default + DeserializeOwned> FromRequest<'a> for PathOption<T> {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> poem::Result<Self> {
        let path = Path::<T>::from_request_without_body(req).await;
        match path {
            Ok(path) => Ok(Self(path.0)),
            Err(_) => Ok(Self(T::default())),
        }
    }
}
