use axum::{routing, Router};

use crate::service::{middleware, SharedConfig};

use super::handler;

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub(crate) fn router(_config: SharedConfig) -> Router {
    let authn_routes = Router::new().route("/authenticate", routing::get(handler::authenticate));

    let operation_routes = Router::new()
        .route("/open_at", routing::post(handler::open_at))
        .layer(axum::middleware::from_fn(middleware::authorize));

    authn_routes.merge(operation_routes)
}
