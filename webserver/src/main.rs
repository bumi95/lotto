use actix_web::{get, web, App, HttpServer, Responder};
use std::{
    cell::Cell,
    sync::atomic::{AtomicUsize, Ordering},
    sync::Arc,
};

#[derive(Clone)]
struct AppState {
    local: Cell<usize>,
    global: Arc<AtomicUsize>,
}

#[get("/")]
async fn show_count(data: web::Data<AppState>) -> impl Responder {
    format!(
        "global count : {}\nlocal count : {}",
        data.global.load(Ordering::Relaxed), data.local.get()
    )
}

#[get("/add")]
async fn add_one(data: web::Data<AppState>) -> impl Responder {
    data.global.fetch_add(1, Ordering::Relaxed);

    let local_cnt = data.local.get();
    data.local.set(local_cnt + 1);

    format!(
        "global count : {}\nlocal count : {}",
        data.global.load(Ordering::Relaxed), data.local.get()
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = AppState {
        local: Cell::new(0),
        global: Arc::new(AtomicUsize::new(0)),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            .service(show_count)
            .service(add_one)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
