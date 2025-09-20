use crate::common::html::context_html::ContextHtmlBuilder;
use crate::common::html::validate::ValidateErrorMessageExt;
use crate::user::form::locale::UserFormLocale;
use crate::user::rule::user_manager::PasswordUserManagerRulesExt;
use cjtoolkit_structured_validator::common::flag_error::FlagCounter;
use cjtoolkit_structured_validator::types::password::{Password, PasswordError};
use maud::{Markup, html};
use poem::i18n::Locale;
use serde::{Deserialize, Serialize};
use shared::locale::LocaleExtForResult;
use std::sync::Arc;

#[derive(Deserialize, Default)]
pub struct EditPasswordManagerForm {
    pub password: String,
    pub password_confirm: String,
    pub csrf_token: String,
}

impl EditPasswordManagerForm {
    pub async fn as_validated(&self) -> EditPasswordManagerResult {
        EditPasswordManagerResult(
            async {
                let mut flag = FlagCounter::new();

                let (password, password_confirm) =
                    Password::parse_password_add(Some(&self.password), &self.password_confirm);
                let password = flag.check(password);
                let password_confirm = flag.check(password_confirm);

                if flag.is_flagged() {
                    return Err(EditPasswordManagerError {
                        password,
                        password_confirm,
                    });
                }

                Ok(EditPasswordManagerValidated {
                    password: password.expect("Password is not empty"),
                    password_confirm: password_confirm.expect("Password Confirm is not empty"),
                })
            }
            .await,
        )
    }

    pub async fn as_form_html(
        &self,
        context_html_builder: &ContextHtmlBuilder,
        errors: Option<EditPasswordManagerMessage>,
        token: Option<Markup>,
    ) -> Markup {
        let errors = errors.unwrap_or_default();
        let token = token.unwrap_or_default();
        let user_form_locale = UserFormLocale::new(&context_html_builder.locale);
        context_html_builder.attach_title(&user_form_locale.title_edit_password).attach_content(html! {
            h1 .mt-3 { (user_form_locale.title_edit_password) }
            form .form {
                (token)
                div .form-group {
                    label for="password" { (user_form_locale.password) }
                    input .form-item type="password" name="password" id="password"
                    placeholder=(user_form_locale.password_placeholder) {}
                    (errors.password.into_error_html())
                }
                div .form-group {
                    label for="password-confirm" { (user_form_locale.password_confirm) }
                    input .form-item type="password" name="password_confirm" id="password-confirm"
                    placeholder=(user_form_locale.password_confirm_placeholder) {}
                    (errors.password_confirm.into_error_html())
                }
                div .form-group {
                    input .btn .btn-sky-blue type="submit" value="Submit" {}
                }
            }
        }).build()
    }
}

pub struct EditPasswordManagerValidated {
    pub password: Password,
    pub password_confirm: Password,
}

pub struct EditPasswordManagerError {
    pub password: Result<Password, PasswordError>,
    pub password_confirm: Result<Password, PasswordError>,
}

impl EditPasswordManagerError {
    pub fn as_message(&self, locale: &Locale) -> EditPasswordManagerMessage {
        EditPasswordManagerMessage {
            password: self.password.as_translated_message(locale),
            password_confirm: self.password_confirm.as_translated_message(locale),
        }
    }
}

pub struct EditPasswordManagerResult(
    pub Result<EditPasswordManagerValidated, EditPasswordManagerError>,
);

#[derive(Debug, Clone, Serialize, Default)]
pub struct EditPasswordManagerMessage {
    pub password: Arc<[String]>,
    pub password_confirm: Arc<[String]>,
}
