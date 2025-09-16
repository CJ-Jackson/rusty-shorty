use error_stack::Report;
use shared::error::boot_error::MainError;

pub async fn boot() -> Result<(), Report<MainError>> {
    Ok(())
}
