use backoffice::export::{MainError, init_log};
use error_stack::fmt::ColorMode;
use error_stack::{Report, ResultExt};

#[tokio::main]
async fn main() -> Result<(), Report<MainError>> {
    init_log();
    Report::set_color_mode(ColorMode::None);

    let handle = tokio::spawn(backoffice::boot());
    public::boot().await?;
    handle.await.change_context(MainError::ThreadError)?
}
