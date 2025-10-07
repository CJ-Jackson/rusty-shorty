use crate::common::embed::Asset;
use maud::{Markup, PreEscaped, html};
use shared::embed::EmbedAsString;

#[inline]
fn js_debug_prod(debug: &str, prod: &str) -> String {
    if cfg!(debug_assertions) {
        Asset::get(debug).as_string()
    } else {
        Asset::get(prod).as_string()
    }
}

pub fn js_main() -> String {
    js_debug_prod("js/main.js", "js/main.min.js")
}

pub fn js_vec_wrap(vec: Vec<String>) -> Markup {
    html! {
        script type="module" { (PreEscaped(vec.join("\n"))) }
    }
}
