// use actix_web::{get, post, web, App, HttpResponse, HttpServer};
// // use actix_web::{rt, web, App, Error, HttpRequest, HttpResponse, HttpServer};
// // use actix_ws::AggregatedMessage;
// // use futures_util::StreamExt as _;
// // mod model;
// // use model::todo::AddTodo;
// // use validator::Validate; 
// // use actix_web::web::Json;
// // use crate::model::todo::AddTodo;
// use actix_web::Responder;
// // This struct represents state
// struct AppState {
//     app_name: String,
// }

// #[get("/")]
// async fn index() -> impl Responder {
//     let app_name = &data.app_name; // <- get app_name
//     // format!("Hello {app_name}!") // <- response with app_name
//     HttpResponse::Ok().body(app_name);
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }
// // #[post("/add")]
// // async fn create(body: Json<AddTodo>) -> Result<Json<AddTodo>, actix_web::Error>{
// //     let isValid = body.validate();
// //     match isValid {
// //         Ok(_) => {
// //             let todo = body.todo.clone();
// //             // HttpResponse::Ok().body(format!("{todo} Todo added"))
            
// //         }
// //         Err(err) => {
// //             let response = HttpResponse::BadRequest().body(err.to_string());
// //             return Err(actix_web::Error::from(response));
// //         }
// //     }
// // }


// // async fn echo(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
// //     let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

// //     let mut stream = stream
// //         .aggregate_continuations()
// //         // aggregate continuation frames up to 1MiB
// //         .max_continuation_size(2_usize.pow(20));

// //     // start task but don't wait for it
// //     rt::spawn(async move {
// //         // receive messages from websocket
// //         while let Some(msg) = stream.next().await {
// //             match msg {
// //                 Ok(AggregatedMessage::Text(text)) => {
// //                     // echo text message
// //                     session.text(text).await.unwrap();
// //                 }

// //                 Ok(AggregatedMessage::Binary(bin)) => {
// //                     // echo binary message
// //                     session.binary(bin).await.unwrap();
// //                 }

// //                 Ok(AggregatedMessage::Ping(msg)) => {
// //                     // respond to PING frame with PONG frame
// //                     session.pong(&msg).await.unwrap();
// //                 }

// //                 _ => {}
// //             }
// //         }
// //     });

// //     // respond immediately with response connected to WS session
// //     Ok(res)
// // }

// // #[actix_web::main]
// // async fn main() -> std::io::Result<()> {

// //     println!("http://127.0.0.1:8081");
// //     HttpServer::new(|| App::new().route("/echo", web::get().to(index)))
// //         .bind(("127.0.0.1", 8080))?
// //         .run()
// //         .await
// // }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/echo", web::get().to(index)) // Call the async handler
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }
// // #[actix_web::main]
// // async fn main() -> std::io::Result<()> {
// //     HttpServer::new(|| App::new().service(index).service(hello))
// //         .bind(("127.0.0.1", 8081))?
// //         .run()
// //         .await
// // }
// // use actix_web::web::Data;
// // use actix_web::{get, patch, post, web::Json, web::Path, App, HttpResponse, HttpServer, Responder};
// // // use error::PizzaError;
// // mod model;
// // use crate::db::{pizza_data_trait::PizzaDataTrait, Database};
// // mod db;
// // use crate::model::todo::{AddTodo};
// // use uuid;
// // use validator::Validate;
// // mod error;

// // #[get("/todos")]
// // async fn get_todos(db: Data<Database>) -> Result<Json<Vec<Todo>>, PizzaError> {
// //     let pizzas = Database::get_all_pizzas(&db).await;
// //     match pizzas {
// //         Some(found_pizzas) => Ok(Json(found_pizzas)),
// //         None => Err(PizzaError::NoPizzasFound),
// //     }
// // }

// // #[post("/addtodo")]
// // async fn add_todo(
// //     body: Json<BuyPizzaRequest>,
// //     db: Data<Database>,
// // ) -> Result<Json<Pizza>, PizzaError> {
// //     let is_valid = body.validate();
// //     match is_valid {
// //         Ok(_) => {
// //             let pizza_name = body.pizza_name.clone();

// //             let mut buffer = uuid::Uuid::encode_buffer();
// //             let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

// //             let new_pizza =
// //                 Database::add_pizza(&db, Pizza::new(String::from(new_uuid), pizza_name)).await;

// //             match new_pizza {
// //                 Some(created) => Ok(Json(created)),
// //                 None => Err(PizzaError::PizzaCreationFailure),
// //             }
// //         }
// //         Err(_) => Err(PizzaError::PizzaCreationFailure),
// //     }
// // }

// // #[patch("/updatetodo/{uuid}")]
// // async fn update_todo(
// //     update_pizza_url: Path<UpdatePizzaURL>,
// //     db: Data<Database>,
// // ) -> Result<Json<Todo>, PizzaError> {
// //     let uuid = update_pizza_url.into_inner().uuid;
// //     let update_result = Database::update_pizza(&db, uuid).await;
// //     match update_result {
// //         Some(updated_pizza) => Ok(Json(updated_pizza)),
// //         None => Err(PizzaError::NoSuchPizzaFound),
// //     }
// // }

// // #[actix_web::main]
// // async fn main() -> std::io::Result<()> {
// //     let db = Database::init()
// //         .await
// //         .expect("error connecting to database");
// //     let db_data = Data::new(db);

// //     HttpServer::new(move || {
// //         App::new()
// //             .app_data(db_data.clone())
// //             .service(get_todos)
// //             .service(add_todo)
// //             .service(update_todo)
// //     })
// //     .bind("127.0.0.1:8081")?
// //     .run()
// //     .await
// // }

// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// // Define the handler function
// #[get("/")]
// async fn index() -> impl Responder {
//     HttpResponse::Ok().body("Hello from Actix Web!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// #[post("/add")]
// async fn create(body: Json<AddTodo>) -> Result<Json<AddTodo>, actix_web::Error>{
//     let isValid = body.validate();
//     match isValid {
//         Ok(_) => {
//             let todo = body.todo.clone();
//             // HttpResponse::Ok().body(format!("{todo} Todo added"))
            
//         }
//         Err(err) => {
//             let response = HttpResponse::BadRequest().body(err.to_string());
//             return Err(actix_web::Error::from(response));
//         }
//     }
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }
// // Main function to start the server
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     println!("127.0.0.1:8080");
//     HttpServer::new(|| {
//         App::new()
//             .route("/index", web::get().to(manual_hello)) // Attach the handler
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

// use actix_web::{web, App, HttpServer, Responder, HttpResponse};
// use std::sync::Mutex;
// use uuid::Uuid;
// use serde::{Deserialize, Serialize};
// // use uuid::Uuid;

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct TodoItem {
//     pub id: Uuid,
//     pub title: String,
//     pub completed: bool,
// }


// // mod models;
// // use ::TodoItem;

// struct AppState {
//     todos: Mutex<Vec<TodoItem>>,
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let state = web::Data::new(AppState {
//         todos: Mutex::new(Vec::new()),
//     });

//     HttpServer::new(move || {
//         App::new()
//             .app_data(state.clone())
//             .route("/todos", web::get().to(get_todos))
//             .route("/todos", web::post().to(create_todo))
//             .route("/todos/{id}", web::delete().to(delete_todo))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

// async fn get_todos(state: web::Data<AppState>) -> impl Responder {
//     let todos = state.todos.lock().unwrap();
//     HttpResponse::Ok().json(&*todos)
// }

// async fn create_todo(state: web::Data<AppState>, todo: web::Json<TodoItem>) -> impl Responder {
//     let mut todos = state.todos.lock().unwrap();
//     let mut new_todo = todo.into_inner();
//     new_todo.id = Uuid::new_v4(); // Assign a unique ID
//     todos.push(new_todo.clone());
//     HttpResponse::Created().json(new_todo)
// }

// async fn delete_todo(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
//     let mut todos = state.todos.lock().unwrap();
//     if let Some(pos) = todos.iter().position(|todo| todo.id == *id) {
//         todos.remove(pos);
//         HttpResponse::NoContent().finish()
//     } else {
//         HttpResponse::NotFound().body("Todo not found")
//     }
// }
use actix_web::{post,get, web, App, HttpServer,HttpResponse, Responder};
// mod model;
// use model::todo::AddTodo;
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("http://127.0.0.1:8081");
    HttpServer::new(|| App::new().service(index)
    // .service(
    //     // prefixes all resources and routes attached to it...
    //     web::scope("/app")
    //     // ...so this handles requests for `GET /app/index.html`
    //     .route("/index.html", web::get().to(index)),
    // )
    .service(hello).route("/hey", web::get().to(manual_hello)))
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}