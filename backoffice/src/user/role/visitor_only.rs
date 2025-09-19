use crate::user::model::user_model::UserIdContext;
use crate::user::role::Role;
use crate::user::route::LOGIN_ROUTE;
use poem::http::StatusCode;
use poem::web::Redirect;
use poem::{Endpoint, Error, FromRequest, IntoEndpoint, IntoResponse, Request};
use shared::context::Dep;

struct VisitorOnly<E: Endpoint>(E);

impl<E: Endpoint> Endpoint for VisitorOnly<E> {
    type Output = E::Output;

    async fn call(&self, req: Request) -> poem::Result<Self::Output> {
        let Dep(user_context) = Dep::<UserIdContext>::from_request_without_body(&req).await?;

        if user_context.role != Role::Visitor {
            return Err(Error::from_status(StatusCode::FORBIDDEN));
        }

        self.0.call(req).await
    }
}

pub fn visitor_only<E>(endpoint: E) -> impl Endpoint
where
    E: IntoEndpoint,
    E::Endpoint: 'static,
{
    VisitorOnly(endpoint.into_endpoint())
}

pub async fn redirect_if_visitor<E: Endpoint>(next: E, req: Request) -> poem::Result<E::Output> {
    if req.uri().path().contains(LOGIN_ROUTE) {
        return next.call(req).await;
    }
    let Dep(user_context) = Dep::<UserIdContext>::from_request_without_body(&req).await?;
    if user_context.role == Role::Visitor {
        return Err(Error::from_response(
            Redirect::see_other(LOGIN_ROUTE).into_response(),
        ));
    }

    next.call(req).await
}
