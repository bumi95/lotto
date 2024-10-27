use actix_cors::Cors;
use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use rand::seq::SliceRandom;
use serde::Serialize;

#[derive(Serialize)]
struct LottoNumbers {
    numbers: Vec<u8>,
}

#[get("/lotto")]
async fn get_lotto_numbers() -> impl Responder {
    let mut numbers: Vec<u8> = (1..46).collect(); // 로또 번호는 1부터 45까지
    let mut rng = rand::thread_rng();
    numbers.shuffle(&mut rng);
    let mut selected_numbers = numbers[..6].to_vec(); // 첫 6개 번호 선택
    selected_numbers.sort();

    HttpResponse::Ok().json(LottoNumbers {
        numbers: selected_numbers,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default().allow_any_origin())
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
