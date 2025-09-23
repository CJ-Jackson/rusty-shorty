use crate::error::LogData;
use log::error;

pub fn init_log() {
    colog::init();
}

pub fn log_poem_error(err: &poem::Error) {
    if let Some(log_data) = err.data::<LogData>() {
        error!("{} - {}", err.status(), &log_data.summary);
    }
}
