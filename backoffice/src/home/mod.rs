use crate::common::embed::AssetFileEndPoint;
use poem::Route;

pub fn home_route() -> Route {
    Route::new().at(
        "/favicon.ico",
        AssetFileEndPoint::new("/favicon/favicon.ico"),
    )
}
