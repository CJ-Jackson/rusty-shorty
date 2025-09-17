use crate::user::model::user_model::UserIdContext;
use crate::user::role::Role;
use crate::user::route::LOGIN_ROUTE;
use poem::http::StatusCode;
use poem::web::Redirect;
use poem::{Endpoint, Error, FromRequest, IntoResponse, Request, Response};
use shared::context::Dep;

struct UserRoleCheck<E: Endpoint>(Role, E);

impl<E: Endpoint> Endpoint for UserRoleCheck<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> poem::Result<Self::Output> {
        let Dep(user_context) = Dep::<UserIdContext>::from_request_without_body(&req).await?;

        if user_context.role == Role::Visitor {
            return Ok(Redirect::see_other(LOGIN_ROUTE).into_response());
        }
        if user_context.role < self.0 {
            return Err(Error::from_status(StatusCode::UNAUTHORIZED));
        }

        Ok(self.1.call(req).await?.into_response())
    }
}

pub fn must_be_user<E: Endpoint>(endpoint: E) -> impl Endpoint {
    UserRoleCheck(Role::User, endpoint)
}

pub fn must_be_root<E: Endpoint>(endpoint: E) -> impl Endpoint {
    UserRoleCheck(Role::Root, endpoint)
}
