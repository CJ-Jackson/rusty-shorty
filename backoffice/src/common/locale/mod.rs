use crate::common::embed::AssetLocale;
use poem::error::I18NError;
use poem::i18n::I18NResources;

pub fn build_locale_resources() -> Result<I18NResources, I18NError> {
    let locale_map = AssetLocale::locale_map();
    let mut resources = I18NResources::builder();
    for (locale, context) in locale_map {
        resources = resources.add_ftl(locale, context)
    }
    resources.build()
}
