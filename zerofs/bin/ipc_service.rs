use std::sync::Arc;

use zerofs::{
    config::ZerofsConfig,
    service::{FsHttpServer, ServiceResult},
};

//--------------------------------------------------------------------------------------------------
// Main
//--------------------------------------------------------------------------------------------------

#[tokio::main]
async fn main() -> ServiceResult<()> {
    tracing_subscriber::fmt::init();
    let _config = Arc::new(ZerofsConfig::default());

    Ok(())
}
