use crate::common::embed::AssetHidden;
use maud::{Markup, PreEscaped};
use shared::embed::EmbedAsString;

pub fn plus_icon() -> Markup {
    PreEscaped(AssetHidden::get("icon/plus.svg").as_string())
}

pub fn pencil_square_icon() -> Markup {
    PreEscaped(AssetHidden::get("icon/pencil-square.svg").as_string())
}

pub fn key_icon() -> Markup {
    PreEscaped(AssetHidden::get("icon/key.svg").as_string())
}

pub fn flag_icon() -> Markup {
    PreEscaped(AssetHidden::get("icon/flag.svg").as_string())
}

pub fn trash_icon() -> Markup {
    PreEscaped(AssetHidden::get("icon/trash.svg").as_string())
}
