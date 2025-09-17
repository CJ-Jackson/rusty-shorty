pub(crate) mod common;
pub(crate) mod user;

use error_stack::Report;
use shared::error::boot_error::MainError;

pub mod error_export {
    pub use shared::error::boot_error::MainError;
}

pub async fn boot() -> Result<(), Report<MainError>> {
    todo!()
}
