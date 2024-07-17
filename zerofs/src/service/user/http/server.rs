use std::sync::Arc;

use tokio::net::TcpListener;

use crate::service::{router, ServiceResult, SharedConfig};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// This is an HTTP/1.1 server for working remotely with `zerofs` files and directories.
///
/// To support all the operations allowed by the zerofs library, the server uses the concept
/// of handles as a way to reference files and directories in subsequent requests. The handles
/// are just a way of identifying files and directories and there is nothing stateful about them.
///
/// File input and output streams are treated as chunks of data with the support of the
/// `Transfer-Encoding: chunked` header.
pub struct FsHttpServer {
    /// The configuration of the file system.
    config: SharedConfig,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl FsHttpServer {
    /// Creates a new HTTP server for the file system service.
    pub fn new(config: SharedConfig) -> Self {
        Self { config }
    }

    /// Starts the HTTP server.
    pub async fn start(&self) -> ServiceResult<()> {
        let router = router::router(Arc::clone(&self.config));
        let listener = TcpListener::bind(self.config.network.get_user_address()).await?;

        tracing::info!(
            "HTTP server started at {}",
            self.config.network.get_user_address()
        );

        axum::serve(listener, router).await?;

        Ok(())
    }
}
