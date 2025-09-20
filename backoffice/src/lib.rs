pub(crate) mod common;
pub(crate) mod home;
pub(crate) mod user;

use crate::common::embed::{AssetFilesEndPoint, EMBED_PATH};
use crate::common::locale::build_locale_resources;
use crate::home::home_route;
use crate::user::model::user_model::UserIdContext;
use crate::user::route::login::login_route;
use crate::user::route::user::{USER_ROUTE, user_route};
use error_stack::{Report, ResultExt};
use poem::listener::TcpListener;
use poem::middleware::{CookieJarManager, Csrf};
use poem::session::{CookieConfig, CookieSession};
use poem::{EndpointExt, Server};
use shared::cache_local::init_cache_local;
use shared::config::Config;
use shared::csrf::{CSRF_PATH, route_csrf};
use shared::error::boot_error::MainError;
use user::route::login::LOGIN_ROUTE;

pub mod error_export {
    pub use shared::error::boot_error::MainError;
}

pub async fn boot() -> Result<(), Report<MainError>> {
    let config = Config::fetch()
        .await
        .change_context(MainError::ConfigError)?;

    let route = home_route();

    let route = route
        .nest(LOGIN_ROUTE, login_route())
        .nest(USER_ROUTE, user_route())
        .nest(CSRF_PATH, route_csrf())
        .nest(EMBED_PATH, AssetFilesEndPoint::new());

    let route = route
        .around(init_cache_local::<UserIdContext, _>)
        .with(CookieJarManager::new())
        .with(CookieSession::new(CookieConfig::new()))
        .with(Csrf::new())
        .data(build_locale_resources().change_context(MainError::LocaleError)?);

    match config.upgrade() {
        Some(config) => {
            println!(
                "Backoffice Listening on http://{}",
                config.poem_backoffice.parse_address()
            );
            Server::new(TcpListener::bind(&config.poem_backoffice.parse_address()))
                .run(route)
                .await
                .change_context(MainError::IoError)
        }
        None => Err(Report::new(MainError::ConfigError)),
    }
}
