use crate::common::html::context_html::ContextHtmlBuilder;
use crate::user::role::visitor_only::visitor_only;
use crate::user::service::user_login_service::UserLoginService;
use error_stack::Report;
use maud::{Markup, html};
use poem::error::ResponseError;
use poem::web::Redirect;
use poem::{IntoResponse, Response, Route, get, handler};
use shared::context::Dep;
use shared::csrf::CsrfError;
use shared::query_string::form::FormQs;

#[handler]
async fn login(Dep(context_html_builder): Dep<ContextHtmlBuilder>) -> Markup {
    let title = context_html_builder
        .locale
        .text("login-title")
        .unwrap_or("Login".to_string());
    let username = context_html_builder
        .locale
        .text("login-username")
        .unwrap_or("Username".to_string());
    let password = context_html_builder
        .locale
        .text("login-password")
        .unwrap_or("Password".to_string());
    let confirm_button = context_html_builder
        .locale
        .text("login-confirm-button")
        .unwrap_or("Login".to_string());
    context_html_builder
        .attach_title(&title)
        .attach_content(html! {
            h1 .mt-3 { (title) }
            form method="post" .form {
                input .form-item type="text" name="username" placeholder=(username) {}
                input .form-item type="password" name="password" placeholder=(password) {}
                button .btn .btn-sky-blue .mt-3 type="submit" { ( confirm_button) }
            }
        })
        .build()
}

enum LoginPostResponse {
    Redirect(Redirect),
    CsrfError(Report<CsrfError>),
}

impl IntoResponse for LoginPostResponse {
    fn into_response(self) -> Response {
        match self {
            Self::Redirect(redirect) => redirect.into_response(),
            Self::CsrfError(csrf) => csrf.current_context().as_response(),
        }
    }
}

// #[handler]
// async fn login_post(Dep(user_login_service): Dep<UserLoginService>, FormQs()) -> LoginPostResponse {
//     todo!()
// }

pub fn login_route() -> Route {
    Route::new().at("/", visitor_only(get(login)))
}
