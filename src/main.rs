mod controller;
pub mod error;
mod server;

use crate::error::RrbgError;
use crate::server::server::start;

#[tokio::main]
async fn main() -> Result<(), RrbgError> {
    start().await;

    Ok(())
}
