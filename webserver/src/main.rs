use actix_cors::Cors;
use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use rand::seq::SliceRandom;
use serde::Serialize;

const LOTTO_MIN: u8 = 1;
const LOTTO_MAX: u8 = 45;
const LOTTO_COUNT: usize = 6;

#[derive(Serialize)]
struct LottoNumbers {
    numbers: Vec<u8>,
}

#[get("/lotto")]
async fn get_lotto_numbers() -> impl Responder {
    let mut numbers: Vec<u8> = (LOTTO_MIN..=LOTTO_MAX).collect();
    let mut rng = rand::thread_rng();
    
    numbers.shuffle(&mut rng);
    let mut selected_numbers = numbers[..LOTTO_COUNT].to_vec();
    selected_numbers.sort();
    HttpResponse::Ok().json(LottoNumbers {
        numbers: selected_numbers,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET"])
                    .allow_any_header()
                    .max_age(3600)
            )
            .service(get_lotto_numbers)
            .service(
                Files::new("/", "./")
                    .index_file("main.html")
                    .show_files_listing(),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
