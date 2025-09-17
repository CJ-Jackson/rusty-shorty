use crate::user::model::user_model::UserIdContext;
use crate::user::role::Role;
use poem::http::StatusCode;
use poem::{Endpoint, Error, FromRequest, IntoResponse, Request, Response};
use shared::context::Dep;

struct VisitorOnly<E: Endpoint>(E);

impl<E: Endpoint> Endpoint for VisitorOnly<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> poem::Result<Self::Output> {
        let Dep(user_context) = Dep::<UserIdContext>::from_request_without_body(&req).await?;

        if user_context.role != Role::Visitor {
            return Err(Error::from_status(StatusCode::FORBIDDEN));
        }

        Ok(self.0.call(req).await?.into_response())
    }
}

pub fn visitor_only<E: Endpoint>(endpoint: E) -> impl Endpoint {
    VisitorOnly(endpoint)
}
