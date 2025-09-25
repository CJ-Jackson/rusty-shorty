use crate::common::embed::AssetHidden;
use maud::{Markup, PreEscaped};
use shared::embed::EmbedAsString;

pub fn plus_icon() -> Markup {
    PreEscaped(AssetHidden::get("icon/plus.svg").as_string())
}

pub fn pencil_square_icon() -> Markup {
    PreEscaped(AssetHidden::get("icon/pencil_square.svg").as_string())
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

pub fn no_symbol_icon() -> Markup {
    PreEscaped(AssetHidden::get("icon/no_symbol.svg").as_string())
}

pub fn document_magnifying_glass_icon() -> Markup {
    PreEscaped(AssetHidden::get("icon/document_magnifying_glass.svg").as_string())
}
