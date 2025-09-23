use crate::common::cache::RequestCacheExt;
use crate::user::role::Role;
use crate::user::service::user_check_service::UserCheckService;
use error_stack::Report;
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
        let request_cache = ctx.request_cache();
        let mut request_cache = request_cache.lock().await;
        let user_id_context = match request_cache.user_id_context.as_ref() {
            None => {
                let user_service: UserCheckService = ctx.inject().await?;
                let user_id_context = user_service.get_user_context();
                request_cache.user_id_context = Some(user_id_context.clone());
                user_id_context
            }
            Some(user_id_context) => user_id_context.clone(),
        };
        drop(request_cache);
        Ok(user_id_context)
    }
}

pub struct IdPassword {
    pub id: i64,
    pub password: Box<[u8]>,
}
