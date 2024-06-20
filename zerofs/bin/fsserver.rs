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

    let config = Arc::new(ZerofsConfig::default());
    let server = FsHttpServer::new(config);
    server.start().await
}
