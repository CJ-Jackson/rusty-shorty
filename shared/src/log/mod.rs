pub mod repository;
pub mod service;

use crate::context::fetch_context;
use crate::error::LogData;
use crate::log::service::error_stack_log_service::ErrorStackLogService;
use log::error;

pub fn init_log() {
    colog::init();
}

pub async fn log_poem_error(err: &poem::Error) {
    if let Some(log_data) = err.data::<LogData>() {
        error!("{} - {}", err.status(), &log_data.summary);
        let error_stack_log_service: ErrorStackLogService = fetch_context()
            .await
            .expect("Should return error stack service");
        _ = error_stack_log_service.log_data(log_data);
    }
}
