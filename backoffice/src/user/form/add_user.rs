use crate::common::html::context_html::ContextHtmlBuilder;
use crate::common::html::validate::ValidateErrorMessageExt;
use crate::user::form::locale::UserFormLocale;
use crate::user::role::Role;
use crate::user::rule::user_manager::{PasswordUserManagerRulesExt, UsernameUserManagerRulesExt};
use cjtoolkit_structured_validator::common::flag_error::FlagCounter;
use cjtoolkit_structured_validator::types::password::{Password, PasswordError};
use cjtoolkit_structured_validator::types::username::{
    IsUsernameTakenAsync, Username, UsernameError,
};
use maud::{Markup, html};
use poem::i18n::Locale;
use serde::{Deserialize, Serialize};
use shared::locale::LocaleExtForResult;
use std::sync::Arc;

#[derive(Deserialize, Default)]
pub struct AddUserForm {
    pub username: String,
    pub password: String,
    pub password_confirm: String,
    pub role: Role,
    pub csrf_token: String,
}

impl AddUserForm {
    pub async fn as_validated<T: IsUsernameTakenAsync>(&self, service: &T) -> AddUserResult {
        AddUserResult(
            async {
                let mut flag = FlagCounter::new();

                let username =
                    flag.check(Username::parse_user_add(Some(&self.username), service, None).await);
                let (password, password_confirm) =
                    Password::parse_password_add(Some(&self.password), &self.password_confirm);
                let password = flag.check(password);
                let password_confirm = flag.check(password_confirm);

                if flag.is_flagged() {
                    return Err(AddUserError {
                        username,
                        password,
                        password_confirm,
                        role: self.role.clone(),
                    });
                }

                Ok(AddUserValidated {
                    username: username.expect("Username is not empty"),
                    password: password.expect("Password is not empty"),
                    password_confirm: password_confirm.expect("Password Confirm is not empty"),
                    role: self.role.clone(),
                })
            }
            .await,
        )
    }

    pub async fn as_form_html(
        &self,
        context_html_builder: &ContextHtmlBuilder,
        errors: Option<AddUserMessage>,
        token: Option<Markup>,
    ) -> Markup {
        let errors = errors.unwrap_or_default();
        let token = token.unwrap_or_default();
        let user_form_locale = UserFormLocale::new(&context_html_builder.locale);
        context_html_builder.attach_title(&user_form_locale.title_add).attach_content(html! {
            h1 .mt-3 { (user_form_locale.title_add) }
            form .form {
                (token)
                div .form-group {
                    label for="username" { (user_form_locale.username) }
                    input .form-item type="text" name="username" id="username" value=(self.username)
                    placeholder=(user_form_locale.username_placeholder) {}
                    (errors.username.into_error_html())
                }
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

pub struct AddUserValidated {
    pub username: Username,
    pub password: Password,
    pub password_confirm: Password,
    pub role: Role,
}

pub struct AddUserError {
    pub username: Result<Username, UsernameError>,
    pub password: Result<Password, PasswordError>,
    pub password_confirm: Result<Password, PasswordError>,
    pub role: Role,
}

impl AddUserError {
    pub fn as_message(&self, locale: &Locale) -> AddUserMessage {
        AddUserMessage {
            username: self.username.as_translated_message(locale),
            password: self.password.as_translated_message(locale),
            password_confirm: self.password_confirm.as_translated_message(locale),
        }
    }
}

pub struct AddUserResult(pub Result<AddUserValidated, AddUserError>);

#[derive(Debug, Clone, Serialize, Default)]
pub struct AddUserMessage {
    pub username: Arc<[String]>,
    pub password: Arc<[String]>,
    pub password_confirm: Arc<[String]>,
}
