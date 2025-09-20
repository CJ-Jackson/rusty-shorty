use crate::common::html::context_html::ContextHtmlBuilder;
use crate::common::html::validate::ValidateErrorMessageExt;
use crate::user::form::locale::UserFormLocale;
use crate::user::role::Role;
use crate::user::rule::user_manager::UsernameUserManagerRulesExt;
use cjtoolkit_structured_validator::common::flag_error::FlagCounter;
use cjtoolkit_structured_validator::types::username::{
    IsUsernameTakenAsync, Username, UsernameError,
};
use maud::{Markup, html};
use poem::i18n::Locale;
use serde::{Deserialize, Serialize};
use shared::locale::LocaleExtForResult;
use std::sync::Arc;

#[derive(Deserialize, Default)]
pub struct EditUserForm {
    username: String,
    role: Role,
    csrf_token: String,
}

impl EditUserForm {
    pub async fn as_validated<T: IsUsernameTakenAsync>(
        &self,
        service: &T,
        current_user_name: &str,
    ) -> EditUserResult {
        EditUserResult(
            async {
                let mut flag = FlagCounter::new();

                let username = flag.check(
                    Username::parse_user_add(
                        Some(&self.username),
                        service,
                        Some(current_user_name),
                    )
                    .await,
                );

                if flag.is_flagged() {
                    return Err(EditUserError {
                        username,
                        role: self.role.clone(),
                    });
                }

                Ok(EditUserValidated {
                    username: username.expect("Username is not empty"),
                    role: self.role.clone(),
                })
            }
            .await,
        )
    }

    pub async fn as_form_html(
        &self,
        context_html_builder: &ContextHtmlBuilder,
        errors: Option<EditUserMessage>,
        token: Option<Markup>,
    ) -> Markup {
        let errors = errors.unwrap_or_default();
        let token = token.unwrap_or_default();
        let user_form_locale = UserFormLocale::new(&context_html_builder.locale);
        context_html_builder.attach_title(&user_form_locale.title_edit).attach_content(html! {
            h1 .mt-3 { (user_form_locale.title_edit) }
            form .form {
                (token)
                div .form-group {
                    label for="username" { (user_form_locale.username) }
                    input .form-item type="text" name="username" id="username" value=(self.username)
                    placeholder=(user_form_locale.username_placeholder) {}
                    (errors.username.into_error_html())
                }
                div .form-group {
                    label for="role" { "Role" }
                    select .form-item name="role" id="role" {
                        (self.role.html_option())
                    }
                }
                div .form-group {
                    input .btn .btn-sky-blue type="submit" value="Add" {}
                }
            }
        }).build()
    }
}

pub struct EditUserValidated {
    username: Username,
    role: Role,
}

pub struct EditUserError {
    username: Result<Username, UsernameError>,
    role: Role,
}

impl EditUserError {
    pub fn as_message(&self, locale: &Locale) -> EditUserMessage {
        EditUserMessage {
            username: self.username.as_translated_message(locale),
        }
    }
}

pub struct EditUserResult(pub Result<EditUserValidated, EditUserError>);

#[derive(Debug, Clone, Serialize, Default)]
pub struct EditUserMessage {
    username: Arc<[String]>,
}
