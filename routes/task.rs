// use actix_web::{get, post, put, delete, web, Responder};
// use crate::handlers::task_handler::*;
// use uuid::Uuid;

// /// Configure task-related routes
// pub fn task_routes(cfg: &mut web::ServiceConfig) {
//     cfg.service(web::resource("/tasks")
//         .route(web::post().to(create_task))
//         .route(web::get().to(get_tasks)));

//     cfg.service(web::resource("/tasks/{task_id}")
//         .route(web::get().to(get_task))
//         .route(web::put().to(update_task))
//         .route(web::delete().to(delete_task)));
// }
