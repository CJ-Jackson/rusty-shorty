use crate::user::model::user_model::{IdPassword, UserIdContext};
use crate::user::role::Role;
use error_stack::{Report, ResultExt};
use rusqlite::{Connection, OptionalExtension, named_params};
use shared::context::{Context, ContextError, FromContext};
use shared::db::SqliteClient;
use std::sync::MutexGuard;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("Query error")]
    QueryError,
    #[error("Row Value error")]
    RowValueError,
    #[error("Lock error")]
    LockError,
    #[error("Not found error")]
    NotFoundError,
}

pub struct UserRepository {
    sqlite_client: SqliteClient,
}

impl UserRepository {
    pub fn new(sqlite_client: SqliteClient) -> Self {
        Self { sqlite_client }
    }

    pub fn add_token(
        &self,
        token: String,
        user_id: i64,
    ) -> Result<(), Report<UserRepositoryError>> {
        let conn = self.borrow_conn()?;

        conn.execute(
            include_str!("_sql/user_repository/add_token.sql"),
            named_params! {
                ":token": token,
                ":user_id": user_id,
            },
        )
        .change_context(UserRepositoryError::QueryError)?;

        Ok(())
    }

    pub fn delete_token(&self, token: String) -> Result<(), Report<UserRepositoryError>> {
        let conn = self.borrow_conn()?;

        conn.execute(
            include_str!("_sql/user_repository/delete_token.sql"),
            named_params! {
                ":token": token,
            },
        )
        .change_context(UserRepositoryError::QueryError)?;

        Ok(())
    }

    pub fn find_by_token(
        &self,
        token: String,
    ) -> Result<UserIdContext, Report<UserRepositoryError>> {
        let conn = self.borrow_conn()?;

        let mut stmt = conn
            .prepare_cached(include_str!("_sql/user_repository/find_by_token.sql"))
            .change_context(UserRepositoryError::QueryError)?;

        let row: Option<UserIdContext> = stmt
            .query_one(
                named_params! {
                    ":token": token,
                },
                |row| {
                    Ok(UserIdContext {
                        id: row.get("id")?,
                        username: row.get("username")?,
                        role: Role::try_from(row.get::<_, String>("role")?.as_str())
                            .unwrap_or_default(),
                    })
                },
            )
            .optional()
            .change_context(UserRepositoryError::RowValueError)?;

        match row {
            Some(row) => Ok(row),
            None => Err(Report::new(UserRepositoryError::NotFoundError)),
        }
    }

    pub fn get_user_password(
        &self,
        username: String,
    ) -> Result<IdPassword, Report<UserRepositoryError>> {
        let conn = self.borrow_conn()?;

        let mut stmt = conn
            .prepare_cached(include_str!("_sql/user_repository/get_user_password.sql"))
            .change_context(UserRepositoryError::QueryError)?;

        let row: Option<IdPassword> = stmt
            .query_one(
                named_params! {
                    ":username": username,
                },
                |row| {
                    Ok(IdPassword {
                        id: row.get("id")?,
                        password: row.get("password")?,
                    })
                },
            )
            .optional()
            .change_context(UserRepositoryError::RowValueError)?;

        match row {
            Some(row) => Ok(row),
            None => Err(Report::new(UserRepositoryError::NotFoundError)),
        }
    }

    fn borrow_conn(&'_ self) -> Result<MutexGuard<'_, Connection>, Report<UserRepositoryError>> {
        let guard = self
            .sqlite_client
            .get_conn()
            .lock()
            .map_err(|_| Report::new(UserRepositoryError::LockError))?;
        Ok(guard)
    }
}

impl FromContext for UserRepository {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        Ok(Self::new(ctx.inject().await?))
    }
}
