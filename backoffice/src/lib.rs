pub(crate) mod common;
pub(crate) mod home;
pub(crate) mod user;

use crate::home::home_route;
use crate::user::model::user_model::UserIdContext;
use error_stack::{Report, ResultExt};
use poem::listener::TcpListener;
use poem::middleware::{CookieJarManager, Csrf};
use poem::session::{CookieConfig, CookieSession};
use poem::{EndpointExt, Server};
use shared::cache_local::init_cache_local;
use shared::config::Config;
use shared::error::boot_error::MainError;
use std::sync::Arc;

pub mod error_export {
    pub use shared::error::boot_error::MainError;
}

pub async fn boot() -> Result<(), Report<MainError>> {
    let config = Config::fetch()
        .await
        .change_context(MainError::ConfigError)?;

    let route = home_route();

    let route = route
        .with(CookieJarManager::new())
        .with(CookieSession::new(CookieConfig::new()))
        .with(Csrf::new())
        .around(init_cache_local::<Arc<UserIdContext>, _>);

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
