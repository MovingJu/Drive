use axum::Json;
use log::info;
use serde_json::json;

#[utoipa::path(
    get,
    path = "/test",
    params(),
    responses(
        (status = 200, body = serde_json::Value, description = "JSON")
    )
)]
pub async fn test() -> Json<serde_json::Value> {
    info!("Handling request.");
    Json(json!({
        "code" : 200,
        "message" : "Server is operating."
    }))
}
