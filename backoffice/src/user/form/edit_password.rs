use crate::user::rule::user_manager::PasswordUserManagerRulesExt;
use cjtoolkit_structured_validator::common::flag_error::FlagCounter;
use cjtoolkit_structured_validator::types::password::{Password, PasswordError};
use poem::i18n::Locale;
use serde::{Deserialize, Serialize};
use shared::locale::LocaleExtForResult;
use std::sync::Arc;

#[derive(Deserialize, Default)]
pub struct EditPasswordForm {
    current_password: String,
    password: String,
    password_confirm: String,
    csrf_token: String,
}

impl EditPasswordForm {
    pub async fn as_validated(&self) -> EditPasswordResult {
        EditPasswordResult(
            async {
                let mut flag = FlagCounter::new();

                let (password, password_confirm) =
                    Password::parse_password_add(Some(&self.password), &self.password_confirm);
                let password = flag.check(password);
                let password_confirm = flag.check(password_confirm);

                if flag.is_flagged() {
                    return Err(EditPasswordError {
                        password,
                        password_confirm,
                    });
                }

                Ok(EditPasswordValidated {
                    password: password.expect("Password is not empty"),
                    password_confirm: password_confirm.expect("Password Confirm is not empty"),
                })
            }
            .await,
        )
    }
}

pub struct EditPasswordValidated {
    password: Password,
    password_confirm: Password,
}

pub struct EditPasswordError {
    password: Result<Password, PasswordError>,
    password_confirm: Result<Password, PasswordError>,
}

impl EditPasswordError {
    pub fn as_message(&self, locale: &Locale) -> EditPasswordMessage {
        EditPasswordMessage {
            password: self.password.as_translated_message(locale),
            password_confirm: self.password_confirm.as_translated_message(locale),
        }
    }
}

pub struct EditPasswordResult(pub Result<EditPasswordValidated, EditPasswordError>);

#[derive(Debug, Clone, Serialize)]
pub struct EditPasswordMessage {
    password: Arc<[String]>,
    password_confirm: Arc<[String]>,
}
