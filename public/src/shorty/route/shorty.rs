use crate::shorty::rule::shorty_path::ShortyPathRuleExt;
use crate::shorty::service::fetch_url_service::FetchUrlService;
use cjtoolkit_structured_validator::types::name::name_alias::Field;
use poem::http::StatusCode;
use poem::web::{Path, Redirect};
use poem::{Error, Route, get, handler};
use shared::context::Dep;
use shared::error::FromErrorStack;

#[handler]
async fn fetch_url(
    Dep(fetch_url_service): Dep<FetchUrlService>,
    Path(path): Path<String>,
) -> poem::Result<Redirect> {
    let path = Field::parse_shorty_path(Some(&path)).map_err(|err| {
        let mut poem_err = Error::from_status(StatusCode::NOT_FOUND);
        poem_err.set_error_message(err.to_string());
        poem_err
    })?;
    let url = fetch_url_service
        .fetch_url(path.as_str())
        .map_err(Error::from_error_stack)?;
    Ok(Redirect::see_other(url.url_redirect))
}

pub fn shorty_route() -> Route {
    Route::new().at("/:path", get(fetch_url))
}
