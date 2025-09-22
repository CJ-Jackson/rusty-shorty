use crate::common::embed::Asset;
use crate::common::html::context_html::ContextHtmlBuilder;
use crate::common::icon::{pencil_square_icon, plus_icon, trash_icon};
use crate::shorty::form::add_edit_url_form::AddEditUrlForm;
use crate::shorty::route::locale::ShortyRouteLocale;
use crate::shorty::service::add_url_service::AddUrlService;
use crate::shorty::service::delete_url_service::DeleteUrlService;
use crate::shorty::service::edit_url_service::EditUrlService;
use crate::shorty::service::list_url_service::ListUrlService;
use crate::user::model::user_model::UserIdContext;
use crate::user::role::Role;
use crate::user::role::user_role_check::must_be_user;
use maud::{Markup, PreEscaped, html};
use poem::http::StatusCode;
use poem::session::Session;
use poem::web::{CsrfToken, CsrfVerifier, Path, Redirect};
use poem::{Error, IntoResponse, Route, get, handler};
use shared::context::Dep;
use shared::csrf::CsrfTokenHtml;
use shared::embed::EmbedAsString;
use shared::error::FromErrorStack;
use shared::flash::{Flash, FlashMessage};
use shared::query_string::form::FormQs;

pub const SHORTY_ROUTE: &str = "/shorty";

#[handler]
async fn list_urls(
    Dep(list_url_service): Dep<ListUrlService>,
    Dep(context_html_builder): Dep<ContextHtmlBuilder>,
    Dep(user_id_context): Dep<UserIdContext>,
) -> Markup {
    let list_urls = list_url_service.list_urls();
    let edit_icon = pencil_square_icon();
    let delete_icon = trash_icon();
    let add_icon = plus_icon();

    let lc = ShortyRouteLocale::new(&context_html_builder.locale);

    context_html_builder
        .attach_title(&lc.title)
        .set_current_tag("shorty")
        .attach_content(html! {
            h1 { (lc.title) }
            table .table-full {
                thead {
                    tr {
                        th { (lc.head_id) }
                        th { (lc.head_path) }
                        th { (lc.head_redirect_url) }
                        th { (lc.head_created_at) }
                        th { (lc.head_created_by) }
                        th .action { (lc.head_action) }
                    }
                }
                tbody {
                    @for url in list_urls.iter() {
                        tr {
                            td { (url.id) }
                            td { (url.url_path) }
                            td { (url.url_redirect) }
                            td .js-date-local { (url.created_at.to_rfc3339()) }
                            td { (url.username) }
                            td .action {
                                @if user_id_context.role == Role::Root || user_id_context.id == url.created_by_user_id {
                                    a .icon href=( format!("{}/edit/{}", SHORTY_ROUTE, url.id)) title=(lc.action_edit) { (edit_icon) }
                                    " "
                                    a .icon .js-delete-confirm data-delete=(url.id)
                                    href=( format!("{}/delete/{}", SHORTY_ROUTE, url.id)) title=(lc.action_delete) { (delete_icon) }
                                }
                            }
                        }
                    }
                }
            }
            div .text-right mt-3 {
                a .inline-block href=( format!("{}/add", SHORTY_ROUTE)) title=(lc.action_add) { (add_icon) }
            }
        }).attach_footer(list_url_js_asset())
        .build()
}

fn list_url_js_asset() -> Markup {
    let js_format_to_local_time = if cfg!(debug_assertions) {
        Asset::get("js/format_to_local_time.js").as_string()
    } else {
        Asset::get("js/format_to_local_time.min.js").as_string()
    };
    let js_delete_confirm = if cfg!(debug_assertions) {
        Asset::get("js/shorty_delete_confirm.js").as_string()
    } else {
        Asset::get("js/shorty_delete_confirm.min.js").as_string()
    };
    html! {
        script type="module"{
            (PreEscaped(format!("{}\n{}", js_format_to_local_time, js_delete_confirm)))
        }
    }
}

enum PostResponse {
    Validation(Markup),
    RedirectSuccess(Redirect),
}

impl IntoResponse for PostResponse {
    fn into_response(self) -> poem::Response {
        match self {
            PostResponse::Validation(validation) => validation
                .with_status(StatusCode::UNPROCESSABLE_ENTITY)
                .into_response(),
            PostResponse::RedirectSuccess(redirect) => redirect.into_response(),
        }
    }
}

#[handler]
async fn edit_url_get(
    Dep(context_html_builder): Dep<ContextHtmlBuilder>,
    Dep(edit_url_service): Dep<EditUrlService>,
    Dep(user_id_context): Dep<UserIdContext>,
    Path(url_id): Path<i64>,
    csrf_token: &CsrfToken,
) -> poem::Result<Markup> {
    let subject_id = edit_url_service
        .fetch_user_id_from_url_id(url_id)
        .map_err(Error::from_error_stack)?;
    if user_id_context.role < Role::Root && user_id_context.id != subject_id.created_by_user_id {
        return Err(Error::from_status(StatusCode::FORBIDDEN));
    }
    let subject_url = edit_url_service
        .get_url_redirect(url_id)
        .map_err(Error::from_error_stack)?;

    let mut edit_url = AddEditUrlForm::default();
    edit_url.url_path = subject_url.url_path;
    edit_url.url_redirect = subject_url.url_redirect;

    Ok(edit_url
        .as_form_html(
            &context_html_builder,
            None,
            Some(csrf_token.as_html()),
            true,
        )
        .await)
}

#[handler]
async fn edit_url_post(
    Dep(context_html_builder): Dep<ContextHtmlBuilder>,
    Dep(edit_url_service): Dep<EditUrlService>,
    Dep(user_id_context): Dep<UserIdContext>,
    Path(url_id): Path<i64>,
    FormQs(edit_url_form): FormQs<AddEditUrlForm>,
    csrf_token: &CsrfToken,
    csrf_verifier: &CsrfVerifier,
    session: &Session,
) -> poem::Result<PostResponse> {
    let subject_id = edit_url_service
        .fetch_user_id_from_url_id(url_id)
        .map_err(Error::from_error_stack)?;
    if user_id_context.role < Role::Root && user_id_context.id != subject_id.created_by_user_id {
        return Err(Error::from_status(StatusCode::FORBIDDEN));
    }
    csrf_verifier
        .validate(edit_url_form.csrf_token.as_str())
        .map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))?;
    let validated_result = edit_url_form.as_validated().await.0;
    match validated_result {
        Ok(validated) => {
            edit_url_service
                .edit_url_submit(&validated, url_id)
                .map_err(Error::from_error_stack)?;
            session.flash(Flash::Success {
                msg: "Successfully edited URL".to_string(),
            });
            Ok(PostResponse::RedirectSuccess(Redirect::see_other(
                SHORTY_ROUTE.to_owned() + "/",
            )))
        }
        Err(error) => {
            let errors = error.as_message(&context_html_builder.locale);
            Ok(PostResponse::Validation(
                edit_url_form
                    .as_form_html(
                        &context_html_builder,
                        Some(errors),
                        Some(csrf_token.as_html()),
                        true,
                    )
                    .await,
            ))
        }
    }
}

#[handler]
async fn add_url_get(
    Dep(context_html_builder): Dep<ContextHtmlBuilder>,
    csrf_token: &CsrfToken,
) -> poem::Result<Markup> {
    let add_url_form = AddEditUrlForm::default();

    Ok(add_url_form
        .as_form_html(
            &context_html_builder,
            None,
            Some(csrf_token.as_html()),
            false,
        )
        .await)
}

#[handler]
async fn add_url_post(
    Dep(context_html_builder): Dep<ContextHtmlBuilder>,
    Dep(add_url_service): Dep<AddUrlService>,
    Dep(user_id_context): Dep<UserIdContext>,
    FormQs(add_url_form): FormQs<AddEditUrlForm>,
    csrf_token: &CsrfToken,
    csrf_verifier: &CsrfVerifier,
    session: &Session,
) -> poem::Result<PostResponse> {
    csrf_verifier
        .validate(add_url_form.csrf_token.as_str())
        .map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))?;
    let validated_result = add_url_form.as_validated().await.0;
    match validated_result {
        Ok(validated) => {
            add_url_service
                .add_url_submit(&validated, user_id_context.id)
                .map_err(Error::from_error_stack)?;
            session.flash(Flash::Success {
                msg: "Successfully added URL".to_string(),
            });
            Ok(PostResponse::RedirectSuccess(Redirect::see_other(
                SHORTY_ROUTE.to_owned() + "/",
            )))
        }
        Err(error) => {
            let errors = error.as_message(&context_html_builder.locale);
            Ok(PostResponse::Validation(
                add_url_form
                    .as_form_html(
                        &context_html_builder,
                        Some(errors),
                        Some(csrf_token.as_html()),
                        false,
                    )
                    .await,
            ))
        }
    }
}

#[handler]
async fn delete_url(
    Dep(delete_url_service): Dep<DeleteUrlService>,
    Dep(user_id_context): Dep<UserIdContext>,
    Path(url_id): Path<i64>,
    session: &Session,
) -> poem::Result<Redirect> {
    let subject_id = delete_url_service
        .fetch_user_id_from_url_id(url_id)
        .map_err(Error::from_error_stack)?;
    if user_id_context.role < Role::Root && user_id_context.id != subject_id.created_by_user_id {
        return Err(Error::from_status(StatusCode::FORBIDDEN));
    }
    delete_url_service
        .delete_url(url_id)
        .map_err(Error::from_error_stack)?;
    session.flash(Flash::Success {
        msg: "Successfully deleted URL".to_string(),
    });
    Ok(Redirect::see_other(SHORTY_ROUTE.to_owned() + "/"))
}

pub fn shorty_route() -> Route {
    Route::new()
        .at("/", must_be_user(get(list_urls)))
        .at(
            "/edit/:url_id",
            must_be_user(get(edit_url_get).post(edit_url_post)),
        )
        .at("/delete/:url_id", must_be_user(get(delete_url)))
        .at("/add", must_be_user(get(add_url_get).post(add_url_post)))
}
