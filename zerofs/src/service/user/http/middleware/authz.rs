use axum::{
    body::Body,
    extract::Request,
    http::{Response, StatusCode},
    middleware::Next,
};

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const AUTHZ_USER_TOKEN_NAME: &str = "x-authz-user-token";

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub(crate) async fn authorize(request: Request, next: Next) -> Result<Response<Body>, StatusCode> {
    // == Session Token ==
    // Extract token from x-authz-user-token http-only cookie.
    // Verify that token has the right delegation chain and session rights. root_user -> user -> server -> user

    // == CSRF Token ==
    // Extract token from x-authz-csrf-token header
    // Extract token from x-authz-csrf-token cookie
    // Verify that token is valid and matches the session token
    Ok(next.run(request).await)
}
