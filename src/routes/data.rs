use axum::{extract::Path, Json};

use log::info;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Person {
    name: String,
    age: u32,
}

/// id 받아서 리턴값 확인해보는 함수
/// - params : id
///
/// 이거 쓰면 swagger에서 보이나?
#[utoipa::path(
    get,
    path = "/data/{id}",
    params(
        ("id" = u32, Path, description = "unique identifier of data")
    ),
    responses(
        (status = 200, body = Person, description = "Person Object as JSON")
    )
)]
pub async fn get_data_by_id(Path(id): Path<u32>) -> Json<Person> {
    let data = Person {
        name: format!("User {id}"),
        age: id * 2,
    };
    info!("{}", format!("Handling request. User : {id}"));
    Json(data)
}
