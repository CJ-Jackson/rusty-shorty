use backoffice::error_export::MainError;
use error_stack::{Report, ResultExt};

#[tokio::main]
async fn main() -> Result<(), Report<MainError>> {
    let handle = tokio::spawn(backoffice::boot());
    public::boot().await?;
    handle.await.change_context(MainError::ThreadError)?
}
