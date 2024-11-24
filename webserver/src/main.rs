use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{get, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use serde::{Deserialize, Serialize};
use reqwest;
use scraper::{Html, Selector};

#[derive(Debug, Serialize, Deserialize)]
struct WinningNumbers {
    #[serde(rename = "returnValue")]
    return_value: String,
    #[serde(rename = "drwNo")]
    drw_no: u32,
    #[serde(rename = "drwNoDate")]
    drw_no_date: String,
    #[serde(rename = "drwtNo1")]
    drwt_no1: u32,
    #[serde(rename = "drwtNo2")]
    drwt_no2: u32,
    #[serde(rename = "drwtNo3")]
    drwt_no3: u32,
    #[serde(rename = "drwtNo4")]
    drwt_no4: u32,
    #[serde(rename = "drwtNo5")]
    drwt_no5: u32,
    #[serde(rename = "drwtNo6")]
    drwt_no6: u32,
    #[serde(rename = "bnusNo")]
    bnus_no: u32,
    #[serde(rename = "firstWinamnt")]
    first_winamnt: u64,
    #[serde(rename = "firstPrzwnerCo")]
    first_przwner_co: u32,
    #[serde(rename = "totSellamnt")]
    tot_sellamnt: u64,
}

#[derive(Debug, Serialize)]
struct WinningStore {
    name: String,
    method: String,
    address: String,
}

#[get("/api/lotto/last-winning")]
async fn get_last_winning() -> impl Responder {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let main_page = client
        .get("https://www.dhlottery.co.kr/gameResult.do?method=byWin")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .send()
        .await;

    let latest_round = match main_page {
        Ok(resp) => {
            let html_content = resp.text().await.unwrap_or_default();
            let document = Html::parse_document(&html_content);
            let selector = Selector::parse("#dwrNoList option:first-child").unwrap();
            
            if let Some(element) = document.select(&selector).next() {
                let text = element.text().collect::<String>();
                text.chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap_or(0)
            } else {
                0
            }
        }
        Err(_) => 0
    };

    if latest_round == 0 {
        return HttpResponse::InternalServerError().finish();
    }

    let url = format!(
        "https://www.dhlottery.co.kr/common.do?method=getLottoNumber&drwNo={}",
        latest_round
    );
    
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .send()
        .await;

    match response {
        Ok(resp) => {
            if let Ok(data) = resp.json::<WinningNumbers>().await {
                HttpResponse::Ok().json(data)
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
        Err(e) => {
            println!("Request failed: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/api/lotto/winning-stores")]
async fn get_winning_stores() -> impl Responder {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let response = client
        .get("https://www.dhlottery.co.kr/store.do?method=topStore&pageGubun=L645&rank=1")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .send()
        .await;

    match response {
        Ok(resp) => {
            let html_content = resp.text().await.unwrap_or_default();
            let document = Html::parse_document(&html_content);
            
            let table_selector = Selector::parse("table.tbl_data_col").unwrap();
            let caption_selector = Selector::parse("caption").unwrap();
            let cell_selector = Selector::parse("td").unwrap();
            let mut stores = Vec::new();

            for table in document.select(&table_selector) {
                if let Some(caption) = table.select(&caption_selector).next() {
                    let caption_text = caption.text().collect::<String>();
                    if caption_text.contains("1등") {
                        let row_selector = Selector::parse("tbody tr").unwrap();
                        for row in table.select(&row_selector) {
                            let cells: Vec<String> = row
                                .select(&cell_selector)
                                .map(|cell| cell.text().collect::<String>().trim().to_string())
                                .collect();

                            if cells.len() >= 4 {
                                stores.push(WinningStore {
                                    name: cells[1].clone(),
                                    method: cells[2].clone(),
                                    address: cells[3].clone(),
                                });
                            }
                        }
                        break;
                    }
                }
            }

            HttpResponse::Ok().json(stores)
        }
        Err(e) => {
            println!("Failed to fetch winning stores: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
            .service(get_last_winning)
            .service(get_winning_stores)
            .service(
                Files::new("/", "../ui/build")
                    .index_file("index.html")
                    .default_handler(|req: ServiceRequest| {
                        let (http_req, _payload) = req.into_parts();
                        async {
                            let response = NamedFile::open("../ui/build/index.html")?
                                .into_response(&http_req);
                            Ok(ServiceResponse::new(http_req, response))
                        }
                    })
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
