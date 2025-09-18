use crate::user::model::user_model::UserIdContext;
use crate::user::role::Role;
use crate::user::route::LOGIN_ROUTE;
use poem::http::StatusCode;
use poem::web::Redirect;
use poem::{Endpoint, Error, FromRequest, IntoEndpoint, IntoResponse, Request, Response};
use shared::context::Dep;

struct UserRoleCheck<E: Endpoint>(Role, E);

impl<E: Endpoint> Endpoint for UserRoleCheck<E> {
    type Output = E::Output;

    async fn call(&self, req: Request) -> poem::Result<Self::Output> {
        let Dep(user_context) = Dep::<UserIdContext>::from_request_without_body(&req).await?;

        if user_context.role == Role::Visitor {
            return Err(Error::from_response(
                Redirect::see_other(LOGIN_ROUTE.to_owned() + "/").into_response(),
            ));
        }
        if user_context.role < self.0 {
            return Err(Error::from_status(StatusCode::UNAUTHORIZED));
        }

        self.1.call(req).await
    }
}

pub fn must_be_user<E>(endpoint: E) -> impl Endpoint
where
    E: IntoEndpoint,
    E::Endpoint: 'static,
{
    UserRoleCheck(Role::User, endpoint.into_endpoint())
}

pub fn must_be_root<E>(endpoint: E) -> impl Endpoint
where
    E: IntoEndpoint,
    E::Endpoint: 'static,
{
    UserRoleCheck(Role::Root, endpoint.into_endpoint())
}
