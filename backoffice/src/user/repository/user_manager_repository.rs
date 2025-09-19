use crate::user::form::add_user::AddUserValidated;
use crate::user::role::Role;
use error_stack::{Report, ResultExt};
use rusqlite::{Connection, named_params};
use shared::context::{Context, ContextError, FromContext};
use shared::db::SqliteClient;
use std::sync::MutexGuard;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserManagerRepositoryError {
    #[error("Query error")]
    QueryError,
    #[error("Row Value error")]
    RowValueError,
    #[error("Lock error")]
    LockError,
    #[error("Not found error")]
    NotFoundError,
}

pub struct UserManagerRepository {
    sqlite_client: SqliteClient,
}

impl UserManagerRepository {
    pub fn new(sqlite_client: SqliteClient) -> Self {
        Self { sqlite_client }
    }

    pub fn add_user(
        &self,
        username: String,
        password: Box<[u8]>,
        role: &Role,
    ) -> Result<(), Report<UserManagerRepositoryError>> {
        let conn = self.borrow_conn()?;

        conn.execute(
            include_str!("_sql/user_manager_repository/add_user.sql"),
            named_params! {
                ":username": username,
                ":password": password,
                ":role": String::from(role),
            },
        )
        .change_context(UserManagerRepositoryError::QueryError)?;
        Ok(())
    }

    pub fn revoke_all_token_by_id(
        &self,
        user_id: i64,
    ) -> Result<(), Report<UserManagerRepositoryError>> {
        let conn = self.borrow_conn()?;

        conn.execute(
            include_str!("_sql/user_manager_repository/revoke_all_token_by_id.sql"),
            named_params! {
                ":user_id": user_id,
            },
        )
        .change_context(UserManagerRepositoryError::QueryError)?;
        Ok(())
    }

    fn borrow_conn(
        &'_ self,
    ) -> Result<MutexGuard<'_, Connection>, Report<UserManagerRepositoryError>> {
        let guard = self
            .sqlite_client
            .get_conn()
            .lock()
            .map_err(|_| Report::new(UserManagerRepositoryError::LockError))?;
        Ok(guard)
    }
}

impl FromContext for UserManagerRepository {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        Ok(Self::new(ctx.inject().await?))
    }
}
