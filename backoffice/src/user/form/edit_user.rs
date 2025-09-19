use crate::user::role::Role;
use crate::user::rule::user_manager::UsernameUserManagerRulesExt;
use cjtoolkit_structured_validator::common::flag_error::FlagCounter;
use cjtoolkit_structured_validator::types::username::{
    IsUsernameTakenAsync, Username, UsernameError,
};
use poem::i18n::Locale;
use serde::{Deserialize, Serialize};
use shared::locale::LocaleExtForResult;
use std::sync::Arc;

#[derive(Deserialize)]
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

#[derive(Debug, Clone, Serialize)]
pub struct EditUserMessage {
    username: Arc<[String]>,
}
