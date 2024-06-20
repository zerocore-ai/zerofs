use axum::Json;

use crate::service::EntityOperation;

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// This endpoint handler is used to open a file at a specific path.
pub(crate) async fn open_at(Json(body): Json<EntityOperation>) -> Json<EntityOperation> {
    println!("OpenAt: {:?}", body);
    Json(body)
}
