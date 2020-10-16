mod server;
pub mod error;
mod controller;

use crate::server::server::start;
use crate::error::RrbgError;

#[tokio::main]
async fn main() -> Result<(), RrbgError> {
    start().await;


    Ok(())
}
