use actix_web::{delete,get, post, put, web, HttpResponse, Responder};
// use surrealdb::key::root::all::new;
// use serde_json::to_string_pretty;
// use utoipa::IntoParams;

use uuid::Uuid;
// use std::sync::MutexGuard;
// use model;
use crate::model::task::{User, Task};
use crate::storeManager::store::AppState;
// use crate::storeManager::db::{Database, User, Task};

use serde::Deserialize;
// use uuid::Uuid;

#[derive(Deserialize)]
struct TaskPath {
    user_id: Uuid,
    task_id: Uuid,
}
// Define a struct for the path parameters
// #[derive(Deserialize, IntoParams)]
// pub struct UserIdPath {
//     pub user_id: Uuid,
// }

// Create a new user
// #[derive(serde::Deserialize)]
#[utoipa::path(
    post,
    path = "/users",
    request_body = User,
    responses(
        (status = 200, description = "User Created successfully"),
        (status = 400, description = "Invalid move")
    )
)]
#[post("/users")]
pub async fn create_user(state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    // println!("name:{}",name);
    let mut users = state.users.lock().unwrap();
    let user_id = Uuid::new_v4();
    let user = User {
        id: user_id,
        name: user.name.clone(),
    };

    users.insert(user_id, user.clone());
    HttpResponse::Created().json(user)
}
// Get all user
// #[derive(serde::Deserialize)]
#[utoipa::path(
    get,
    path = "/users",
    request_body = AppState,
    responses(
        (status = 200, description = "User Created successfully"),
        (status = 400, description = "Invalid move")
    )
)]
#[get("/users")]
pub async fn get_users(state: web::Data<AppState>) -> impl Responder {
    let users_lock = state.users.lock().unwrap();
    if users_lock.is_empty() {
        HttpResponse::NotFound().body("No users found")
    } else {
        HttpResponse::Ok().json(&*users_lock)
    }
}

// Create a task
// #[derive(serde::Deserialize)]
#[utoipa::path(post, path = "/tasks/{user_id}", responses(
    (status = 200, description = "Game status retrieved")
))]
#[post("/tasks/{user_id}")]
pub async fn create_task(
    state: web::Data<AppState>, 
    user_id: web::Path<Uuid>, 
    task: web::Json<Task>
) -> impl Responder {
    // println!("name:{}",user_id);
    let mut tasks = state.tasks.lock().unwrap();
    let user_tasks = tasks.entry(*user_id).or_insert(Vec::new());
    let new_task = Task {
        id: Uuid::new_v4(),
        title: task.title.clone(),
        description: task.description.clone(),
        user: task.user.clone(),
        due_date: task.due_date.clone(),
        status: task.status.clone(),
    };
    

    user_tasks.push(new_task.clone());
    // println!("{}",new_task);
    println!("{}",serde_json::to_string_pretty(&user_tasks).unwrap());
    HttpResponse::Created().json(new_task)
}

// Get all tasks for a user
// #[derive(serde::Deserialize)]
#[utoipa::path(get, path = "/tasks/{user_id}", responses(
    (status = 200, description = "Game status retrieved")
))]
#[get("/tasks/{user_id}")]
pub async fn get_tasks(state: web::Data<AppState>, user_id: web::Path<Uuid>) -> impl Responder {
    let tasks = state.tasks.lock().unwrap();
    println!("tasks:");
    // println!("name:{}",user_id);
    match tasks.get(&user_id) {
        Some(task_list) => HttpResponse::Ok().json(task_list),
        None => HttpResponse::NotFound().body("User has no tasks"),
    }
}

// Update a task
// #[derive(serde::Deserialize)]
#[utoipa::path(put, path = "/users/{user_id}/tasks", responses(
    (status = 200, description = "Game status retrieved")
))]
#[put("/users/{user_id}/tasks/{task_id}")]
pub async fn update_task(
    state: web::Data<AppState>,
    path: web::Path<TaskPath>,
    updated_task: web::Json<Task>,
) -> impl Responder {
    let TaskPath {user_id, task_id} = path.into_inner();
    let mut tasks = state.tasks.lock().unwrap();

    if let Some(user_tasks) = tasks.get_mut(&user_id) {
        if let Some(task) = user_tasks.iter_mut().find(|t| t.id == task_id) {
            task.title = updated_task.title.clone();
            task.description = updated_task.description.clone();
            task.due_date = updated_task.due_date.clone();
            task.status = updated_task.status.clone();

            return HttpResponse::Ok().json(task);
        }
    }

    HttpResponse::NotFound().body("Task not found")
}

// #[utoipa::path(delete, path = "/users/{user_id}/tasks", responses(
//     (status = 200, description = "Game status retrieved")
// ))]
// #[delete("/users/{user_id}/tasks/{task_id}")]
// pub async fn delete_task(
//     state: web::Data<AppState>,
//     path: web::Path<(Uuid, Uuid)>,
// ) -> impl Responder {
//     let (user_id, task_id) = path.into_inner();
//     let mut tasks = state.tasks.lock().unwrap();

//     if let Some(user_tasks) = tasks.get_mut(&user_id) {
//         if let Some(pos) = user_tasks.iter().position(|t| t.id == task_id) {
//             user_tasks.remove(pos);
//             return HttpResponse::NoContent().finish();
//         }
//     }

//     HttpResponse::NotFound().body("Task not found")
// }

// Delete a task
// #[derive(serde::Deserialize)]
#[utoipa::path(
    delete,
    path = "/users/{user_id}/tasks/{task_id}",
    responses(
        (status = 204, description = "Task deleted successfully"),
        (status = 404, description = "Task not found")
    )
)]
#[delete("/users/{user_id}/tasks/{task_id}")]
pub async fn delete_task(
    state: web::Data<AppState>,
    path: web::Path<TaskPath>,
) -> impl Responder {
    let TaskPath {user_id, task_id} = path.into_inner();
    
    let mut tasks = state.tasks.lock().unwrap();

    if let Some(user_tasks) = tasks.get_mut(&user_id) {
        if let Some(pos) = user_tasks.iter().position(|t| t.id == task_id) {
            user_tasks.remove(pos);
            return HttpResponse::Ok().body("Task deleted successfully");
        }
    }

    HttpResponse::NotFound().body("Task not found")
}
// pub mod store;

