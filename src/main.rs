mod api;

use actix_web::{web, App, HttpServer};
use api::task::{get_task, set_task, TaskModal};
use env_logger;
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let tasks_data = Arc::new(Mutex::new(vec![TaskModal {
        text: "Go for zuhur prayer".to_string(),
        user_name: "Raj".to_string(),
    }]));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tasks_data.clone()))
            .service(get_task)
            .service(set_task)
    })
    .bind(("127.0.0.1", 80))
    .unwrap()
    .run()
    .await
}
