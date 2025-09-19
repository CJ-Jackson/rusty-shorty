use crate::user::form::add_user::AddUserValidated;
use crate::user::model::user_manager_model::{FetchUser, ListUser};
use crate::user::role::Role;
use error_stack::{Report, ResultExt};
use rusqlite::{Connection, OptionalExtension, named_params};
use shared::context::{Context, ContextError, FromContext};
use shared::db::SqliteClient;
use std::sync::{Arc, MutexGuard};
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
                ":role": role.as_stringed(),
            },
        )
        .change_context(UserManagerRepositoryError::QueryError)?;
        Ok(())
    }

    pub fn edit_password(
        &self,
        id: i64,
        password: Box<[u8]>,
    ) -> Result<(), Report<UserManagerRepositoryError>> {
        let conn = self.borrow_conn()?;

        conn.execute(
            include_str!("_sql/user_manager_repository/edit_password.sql"),
            named_params! {
                ":id": id,
                ":password": password,
            },
        )
        .change_context(UserManagerRepositoryError::QueryError)?;
        Ok(())
    }

    pub fn edit_user(
        &self,
        id: i64,
        username: String,
        role: &Role,
    ) -> Result<(), Report<UserManagerRepositoryError>> {
        let conn = self.borrow_conn()?;
        conn.execute(
            include_str!("_sql/user_manager_repository/edit_user.sql"),
            named_params! {
                ":id": id,
                ":username": username,
                ":role": role.as_stringed(),
            },
        )
        .change_context(UserManagerRepositoryError::QueryError)?;
        Ok(())
    }

    pub fn fetch_user(
        &self,
        id: i64,
    ) -> Result<Option<FetchUser>, Report<UserManagerRepositoryError>> {
        let conn = self.borrow_conn()?;
        let mut stmt = conn
            .prepare_cached(include_str!("_sql/user_manager_repository/fetch_user.sql"))
            .change_context(UserManagerRepositoryError::QueryError)?;
        let row: Option<FetchUser> = stmt
            .query_one(
                named_params! {
                    ":id": id
                },
                |row| {
                    Ok(FetchUser {
                        username: row.get("username")?,
                        role: Role::try_from(row.get::<_, String>("role")?.as_str())
                            .unwrap_or_default(),
                    })
                },
            )
            .optional()
            .change_context(UserManagerRepositoryError::RowValueError)?;
        Ok(row)
    }

    pub fn list_users(&self) -> Result<Arc<[ListUser]>, Report<UserManagerRepositoryError>> {
        let conn = self.borrow_conn()?;
        let mut stmt = conn
            .prepare_cached(include_str!("_sql/user_manager_repository/list_users.sql"))
            .change_context(UserManagerRepositoryError::QueryError)?;
        let rows = stmt
            .query_map(named_params! {}, |row| {
                Ok(ListUser {
                    id: row.get("id")?,
                    username: row.get("username")?,
                    role: Role::try_from(row.get::<_, String>("role")?.as_str())
                        .unwrap_or_default(),
                })
            })
            .change_context(UserManagerRepositoryError::RowValueError)?;

        let users = rows
            .collect::<Result<Vec<_>, _>>()
            .change_context(UserManagerRepositoryError::RowValueError)?;

        Ok(users.into())
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
