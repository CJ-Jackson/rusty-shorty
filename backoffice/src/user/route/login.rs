use crate::common::html::context_html::ContextHtmlBuilder;
use crate::user::LOGIN_TOKEN_COOKIE_NAME;
use crate::user::form::login::{UserLoginForm, UserLoginFormResult};
use crate::user::role::user_role_check::must_be_user;
use crate::user::role::visitor_only::visitor_only;
use crate::user::route::LOGIN_ROUTE;
use crate::user::service::user_login_service::UserLoginService;
use chrono::TimeDelta;
use error_stack::Report;
use maud::{Markup, html};
use poem::error::ResponseError;
use poem::session::Session;
use poem::web::cookie::{Cookie, CookieJar};
use poem::web::{CsrfToken, CsrfVerifier, Redirect};
use poem::{IntoResponse, Response, Route, get, handler};
use shared::adapter::unified;
use shared::context::Dep;
use shared::cookie_builders::CookieBuilderExt;
use shared::csrf::{CsrfError, CsrfTokenHtml, CsrfVerifierError};
use shared::flash::{Flash, FlashMessage};
use shared::query_string::form::FormQs;

#[handler]
async fn login(
    Dep(context_html_builder): Dep<ContextHtmlBuilder>,
    csrf_token: &CsrfToken,
) -> Markup {
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
                (csrf_token.as_html())
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

#[handler]
async fn login_post(
    Dep(user_login_service): Dep<UserLoginService>,
    FormQs(user_login_form): FormQs<UserLoginForm>,
    session: &Session,
    cookie_jar: &CookieJar,
    csrf_verifier: &CsrfVerifier,
) -> LoginPostResponse {
    unified(async {
        csrf_verifier
            .verify(user_login_form.csrf_token.as_str())
            .map_err(|err| LoginPostResponse::CsrfError(err))?;
        if let UserLoginFormResult(Ok(user_login_form_validated)) = user_login_form.as_validated() {
            let token = user_login_service.validate_login(
                user_login_form_validated.username.as_str().to_string(),
                user_login_form_validated.password.as_str().to_string(),
            );
            if let Some(token) = token {
                let new_cookie = Cookie::new_with_str(LOGIN_TOKEN_COOKIE_NAME, token)
                    .into_builder()
                    .path("/")
                    .expires_by_delta(TimeDelta::days(30))
                    .build();

                cookie_jar.add(new_cookie);
                session.flash(Flash::Success {
                    msg: "Login success".to_string(),
                });
                return Ok(LoginPostResponse::Redirect(Redirect::see_other("/")));
            }
        }

        session.flash(Flash::Error {
            msg: "Login failed".to_string(),
        });
        Err(LoginPostResponse::Redirect(Redirect::see_other(
            LOGIN_ROUTE.to_owned() + "/",
        )))
    })
    .await
}

#[handler]
async fn logout(
    Dep(user_login_service): Dep<UserLoginService>,
    session: &Session,
    cookie_jar: &CookieJar,
) -> Redirect {
    user_login_service.logout();
    cookie_jar.remove(LOGIN_TOKEN_COOKIE_NAME);
    session.flash(Flash::Success {
        msg: "Logout success".to_string(),
    });
    Redirect::see_other("/")
}

pub fn login_route() -> Route {
    Route::new()
        .at("/", visitor_only(get(login).post(login_post)))
        .at("/logout", must_be_user(get(logout)))
}
