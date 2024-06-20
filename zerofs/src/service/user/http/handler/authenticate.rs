use axum::http::{HeaderMap, StatusCode};

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const AUTHN_USER_TOKEN: &str = "x-authn-user-token";
const AUTHN_USER_TOKEN_PROOF_MAP: &str = "x-authn-user-token-proof-map";

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// This handler authenticates a user verifying that the user is allowed to access resources of the server.
/// This is determined by checking that the expected user [`UCAN`][ucan] is issued to the server.
///
/// The server validates the UCAN and return a signed session token to the user as an http-only cookie.
///
/// In addition to that, the server will also return a CSRF token to the user as a cookie which will be expected
/// in a double submit pattern from the user in subsequent requests.
///
/// [ucan]: https://github.com/ucan-wg/spec
pub(crate) async fn authenticate(headers: HeaderMap) -> Result<String, StatusCode> {
    let _user_token = headers
        .get(AUTHN_USER_TOKEN)
        .ok_or(StatusCode::UNAUTHORIZED)? // TODO: Should be a 401 error with message indicating missing token
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?; // TODO: Should be a 400 error with message indicating invalid token

    let _token_proof_map = headers
        .get(AUTHN_USER_TOKEN_PROOF_MAP)
        .ok_or(StatusCode::UNAUTHORIZED)? // TODO: Should be a 401 error with message indicating missing token
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?; // TODO: Should be a 400 error with message indicating invalid token

    // // TODO: Verify the user token delegation chain and rights
    // let token_map: BTreeMap<String, String> = serde_json::from_str(token_store).map_err(|_| StatusCode::BAD_REQUEST)?; // TODO: Should be a 400 error with message indicating invalid token
    // let token_store = MemoryIpldStore::from(token_map);

    // // TODO: Verify the user token delegation chain and rights
    // let ucan = SignedUcan::with_store(user_token, store).map_err(|_| StatusCode::BAD_REQUEST)?; // TODO: Should be a 400 error with message indicating invalid token
    // ucan.verify(ambient_context).map_err(|_| StatusCode::UNAUTHORIZED)?; // TODO: Should be a 401 error with message indicating invalid token

    // // TODO: Issue a session token and CSRF token to the user
    // let session_token = Ucan::builder()
    //      .derive(&[ucan])
    //      .capabilities(capabilities![])
    //      .sign(key_pair)
    //      .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // TODO: Should be a 500 error with message indicating internal server error

    todo!()
}
