use crate::user::role::Role;
use crate::user::rule::user_manager::{PasswordUserManagerRulesExt, UsernameUserManagerRulesExt};
use cjtoolkit_structured_validator::common::flag_error::FlagCounter;
use cjtoolkit_structured_validator::types::password::{Password, PasswordError};
use cjtoolkit_structured_validator::types::username::{
    IsUsernameTakenAsync, Username, UsernameError,
};
use poem::i18n::Locale;
use serde::{Deserialize, Serialize};
use shared::locale::LocaleExtForResult;
use std::sync::Arc;

#[derive(Deserialize, Default)]
pub struct AddUserForm {
    username: String,
    password: String,
    password_confirm: String,
    role: Role,
    csrf_token: String,
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
}

pub struct AddUserValidated {
    username: Username,
    password: Password,
    password_confirm: Password,
    role: Role,
}

pub struct AddUserError {
    username: Result<Username, UsernameError>,
    password: Result<Password, PasswordError>,
    password_confirm: Result<Password, PasswordError>,
    role: Role,
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

#[derive(Debug, Clone, Serialize)]
pub struct AddUserMessage {
    username: Arc<[String]>,
    password: Arc<[String]>,
    password_confirm: Arc<[String]>,
}
