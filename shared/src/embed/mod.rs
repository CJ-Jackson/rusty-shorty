use poem::http::Uri;
use poem::{Endpoint, IntoEndpoint, Request};
use rust_embed::EmbeddedFile;

pub trait EmbedAsString {
    fn as_string(&self) -> String;
}

impl EmbedAsString for Option<EmbeddedFile> {
    fn as_string(&self) -> String {
        self.as_ref()
            .map(|f| String::from_utf8(f.data.to_vec()).unwrap_or_default())
            .unwrap_or_default()
    }
}

struct EnforceMinJsOnProd<EP: Endpoint>(EP);

impl<EP: Endpoint> Endpoint for EnforceMinJsOnProd<EP> {
    type Output = EP::Output;

    async fn call(&self, mut req: Request) -> poem::Result<Self::Output> {
        if cfg!(debug_assertions)
            || !req.uri().path().starts_with("/js/")
            || req.uri().path().to_lowercase().ends_with(".min.js")
            || !req.uri().path().to_lowercase().ends_with(".js")
        {
            return self.0.call(req).await;
        }

        let new_uri = {
            let uri = std::mem::take(req.uri_mut());
            let mut uri_parts = uri.into_parts();
            let path = uri_parts
                .path_and_query
                .as_ref()
                .expect("path_and_query")
                .path();
            let path = if path.ends_with(".js") {
                let path = path.strip_suffix(".js").expect("strip_suffix");
                format!("{}.min.js", path)
            } else if path.ends_with(".JS") {
                let path = path.strip_suffix(".JS").expect("strip_suffix");
                format!("{}.MIN.JS", path)
            } else {
                path.to_string()
            };
            let query = uri_parts
                .path_and_query
                .as_ref()
                .expect("path_and_query")
                .query()
                .unwrap_or_default()
                .to_string();
            let path = format!("{}?{}", path, query);
            uri_parts.path_and_query = Some(path.parse().expect("parse"));
            Uri::from_parts(uri_parts).expect("Uri::from_parts")
        };
        *req.uri_mut() = new_uri;

        self.0.call(req).await
    }
}

pub fn enforce_min_js_on_prod<EP: IntoEndpoint>(ep: EP) -> impl Endpoint {
    EnforceMinJsOnProd(ep.into_endpoint())
}
