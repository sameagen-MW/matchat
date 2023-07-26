use actix_web::{
    get, post, web, App, HttpServer, Responder, HttpResponse, http::header::ContentType
};
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::time::{sleep, Duration};
use matchat::*;

type Directory = HashMap<String, Client>;

struct AppState {
    dir: Mutex<Directory>,
    messages: Mutex<Vec<Message>>,
    current_id: Mutex<u32>,
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/register/{name}")]
async fn register(data: web::Data<AppState>, name: web::Path<String>) -> impl Responder {
    println!("register!");
    let mut dir = data.dir.lock().unwrap();
    dir.insert(name.clone(), Client { name: name.clone(), addr: "address".to_string() });
    println!("Directory: {:?}", dir);
    format!("Register {name}!")
}

#[post("/send")]
async fn send(data: web::Data<AppState>, message: web::Json<Message>) -> impl Responder {
    println!("triggered");
    let mut messages = data.messages.lock().unwrap();
    let mut id = data.current_id.lock().unwrap();

    let mut message = message.into_inner();
    message.id = *id;
    *id += 1;

    messages.push(message);
    println!("Messages: {:?}", messages);
    format!("Send!")
}

#[get("/get/{last}")]
async fn get(data: web::Data<AppState>, last: web::Path<u32>) -> impl Responder {
    loop {
        sleep(Duration::from_millis(100)).await;

        type Messages = Vec<Message>;

        let messages = data.messages.lock().unwrap();
        let l = last.clone();
        let new_messages: Messages = (*messages).clone().into_iter().filter(|x| x.id > l).collect();

        if new_messages.len() > 0 {
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&new_messages).unwrap());
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        dir: Mutex::new(HashMap::new()),
        messages: Mutex::new(Vec::new()),
        current_id: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(get)
            .service(send)
            .service(register)
            .service(greet)
    })
    .bind(("0.0.0.0", 1010))?
    .run()
    .await
}
