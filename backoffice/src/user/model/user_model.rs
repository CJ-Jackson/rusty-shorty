use crate::user::role::Role;
use crate::user::service::user_check_service::UserCheckService;
use error_stack::Report;
use shared::cache_local::CacheLocalRequestExt;
use shared::context::{Context, ContextError, FromContext};

#[derive(Debug, Clone)]
pub struct UserIdContext {
    #[allow(dead_code)]
    pub id: i64,
    pub username: String,
    pub role: Role,
}

impl FromContext for UserIdContext {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        let user_id_context = match ctx.req.cache_local::<UserIdContext>() {
            Some(once_user_id_context) => {
                let v: Result<&UserIdContext, Report<ContextError>> = once_user_id_context
                    .0
                    .get_or_try_init(|| async {
                        let user_service: UserCheckService = ctx.inject().await?;
                        Ok(user_service.get_user_context())
                    })
                    .await;
                v
            }
            None => return Err(Report::new(ContextError::Other)),
        };
        Ok(user_id_context?.clone())
    }
}

pub struct IdPassword {
    pub id: i64,
    pub password: Box<[u8]>,
}
