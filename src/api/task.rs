use std::sync::{Arc, Mutex};

use actix_web::web::{Data, Json};
use actix_web::{get, post};
use serde::{Deserialize, Serialize};

type TaskDataType = Arc<Mutex<Vec<TaskModal>>>;

#[derive(Serialize, Deserialize, Clone)]
pub struct TaskModal {
    pub text: String,
    pub user_name: String,
}

#[post("/task")]
pub async fn set_task(body: Json<TaskModal>, data: Data<TaskDataType>) -> Json<Vec<TaskModal>> {
    data.lock().unwrap().push(body.into_inner());
    return Json(data.lock().unwrap().to_vec());
}

#[get("/task")]
pub async fn get_task(data: Data<TaskDataType>) -> Json<Vec<TaskModal>> {
    Json(data.lock().unwrap().to_vec())
}
