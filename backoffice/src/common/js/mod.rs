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

pub fn confirm_message() -> String {
    js_debug_prod("js/confirm_message.js", "js/confirm_message.min.js")
}

pub fn format_to_local_time() -> String {
    js_debug_prod(
        "js/format_to_local_time.js",
        "js/format_to_local_time.min.js",
    )
}

pub fn js_vec_wrap(vec: Vec<String>) -> Markup {
    html! {
        script type="module"{
            (PreEscaped(vec.join("\n")))
        }
    }
}
