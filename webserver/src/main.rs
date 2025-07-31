use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use serde::{Deserialize, Serialize};
use reqwest;
use scraper::{Html, Selector};
use tokio::time::{self, Duration};
use chrono::{Datelike, Local, Weekday};
use std::sync::Arc;
use tokio_rusqlite::{Connection, Error};
use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule};

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

// DB에서 가져올 데이터를 위한 구조체
#[derive(Debug, Serialize)]
struct WinningNumbersDB {
    draw_no: u32,
    draw_date: String,
    num1: u32,
    num2: u32,
    num3: u32,
    num4: u32,
    num5: u32,
    num6: u32,
    bonus: u32,
}

struct Database {
    conn: Connection,
}

impl Database {
    async fn new() -> Result<Self, Error> {
        let conn = Connection::open("lotto.db").await?;
        
        conn.call(|conn| {
            Ok(conn.execute(
                "CREATE TABLE IF NOT EXISTS lotto_numbers (
                    draw_no INTEGER PRIMARY KEY,
                    draw_date TEXT NOT NULL,
                    num1 INTEGER NOT NULL,
                    num2 INTEGER NOT NULL,
                    num3 INTEGER NOT NULL,
                    num4 INTEGER NOT NULL,
                    num5 INTEGER NOT NULL,
                    num6 INTEGER NOT NULL,
                    bonus INTEGER NOT NULL,
                    prize_amount INTEGER NOT NULL,
                    winners INTEGER NOT NULL
                )",
                [],
            )?)
        }).await?;
        
        Ok(Database { conn })
    }

    async fn insert_winning_numbers(&self, numbers: WinningNumbers) -> Result<(), Error> {
        self.conn.call(move |conn| {
            Ok(conn.execute(
                "INSERT OR REPLACE INTO lotto_numbers 
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                (
                    numbers.drw_no,
                    &numbers.drw_no_date,
                    numbers.drwt_no1,
                    numbers.drwt_no2,
                    numbers.drwt_no3,
                    numbers.drwt_no4,
                    numbers.drwt_no5,
                    numbers.drwt_no6,
                    numbers.bnus_no,
                    numbers.first_winamnt,
                    numbers.first_przwner_co,
                ),
            )?)
        }).await?;
        Ok(())
    }

    async fn get_latest_draw_no(&self) -> Result<Option<u32>, Error> {
        self.conn.call(|conn| {
            let mut stmt = conn.prepare("SELECT MAX(draw_no) FROM lotto_numbers")?;
            let mut rows = stmt.query([])?;
            
            if let Some(row) = rows.next()? {
                Ok(row.get(0)?)
            } else {
                Ok(None)
            }
        }).await
    }

    async fn get_all_winning_numbers(&self) -> Result<Vec<WinningNumbersDB>, Error> {
        self.conn.call(|conn| {
            let mut stmt = conn.prepare(
                "SELECT draw_no, draw_date, num1, num2, num3, num4, num5, num6, bonus 
                 FROM lotto_numbers 
                 ORDER BY draw_no"
            )?;
            
            let rows = stmt.query_map([], |row| {
                Ok(WinningNumbersDB {
                    draw_no: row.get(0)?,
                    draw_date: row.get(1)?,
                    num1: row.get(2)?,
                    num2: row.get(3)?,
                    num3: row.get(4)?,
                    num4: row.get(5)?,
                    num5: row.get(6)?,
                    num6: row.get(7)?,
                    bonus: row.get(8)?,
                })
            })?;

            let mut numbers = Vec::new();
            for row in rows {
                numbers.push(row?);
            }
            
            Ok(numbers)
        }).await
    }
}

async fn fetch_winning_numbers(client: &reqwest::Client, draw_no: u32) -> Option<WinningNumbers> {
    let url = format!(
        "https://www.dhlottery.co.kr/common.do?method=getLottoNumber&drwNo={}",
        draw_no
    );
    
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .send()
        .await
        .ok()?;

    response.json::<WinningNumbers>().await.ok()
}

async fn update_database(db: Arc<Database>) {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    // 현재 DB에 저장된 최신 회차 확인
    let start_draw = match db.get_latest_draw_no().await.unwrap_or(None) {
        Some(num) => num + 1,
        None => 1, // DB가 비어있으면 1회차부터 시작
    };

    // 최신 회차 확인
    let latest_round = match get_latest_round(&client).await {
        Some(num) => num,
        None => {
            eprintln!("최신 회차를 가져오는데 실패했습니다.");
            return;
        }
    };

    // 누락된 회차 데이터 가져오기
    for draw_no in start_draw..=latest_round {
        if let Some(numbers) = fetch_winning_numbers(&client, draw_no).await {
            if let Err(e) = db.insert_winning_numbers(numbers).await {
                eprintln!("회차 {} 저장 실패: {}", draw_no, e);
            } else {
                println!("회차 {} 저장 완료", draw_no);
            }
        }
        // API 호출 간격 조절
        time::sleep(Duration::from_millis(100)).await;
    }
}

async fn schedule_updates(db: Arc<Database>) {
    loop {
        let now = Local::now();
        
        // 이번 주 토요일 21시 계산
        let days_until_saturday = (Weekday::Sat.num_days_from_sunday() + 7 - now.weekday().num_days_from_sunday()) % 7;
        let target = now
            .date_naive()
            .checked_add_days(chrono::Days::new(days_until_saturday as u64))
            .unwrap()
            .and_hms_opt(21, 0, 0)
            .unwrap();

        let wait_time = if now.naive_local() >= target {
            // 이번 주 토요일 21시가 지났다면 다음 주 토요일까지 대기
            target + chrono::Duration::weeks(1) - now.naive_local()
        } else {
            target - now.naive_local()
        };

        println!("다음 업데이트까지 대기 중... ({:?})", wait_time);
        time::sleep(Duration::from_secs(wait_time.num_seconds() as u64)).await;

        println!("데이터베이스 업데이트 시작");
        update_database(db.clone()).await;
    }
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

#[get("/api/lotto/db-chart")]
async fn get_db_chart(db: web::Data<Arc<Database>>) -> impl Responder {
    match db.get_all_winning_numbers().await {
        Ok(numbers) => HttpResponse::Ok().json(numbers),
        Err(e) => {
            eprintln!("DB 데이터 조회 실패: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// 로또 번호 생성 결과를 위한 구조체
#[derive(Serialize)]
struct GeneratedNumbers {
    numbers: Vec<u32>
}

async fn call_predictor_pyo3(past_draws: Vec<Vec<u32>>) -> Option<Vec<u32>> {
    Python::with_gil(|py| {
        // predictor.py 모듈 import
        let predictor = PyModule::from_code_bound(
            py,
            &std::fs::read_to_string("predictor.py").ok()?,
            "predictor.py",
            "predictor"
        ).ok()?;

        // Rust Vec<Vec<u32>> → Python List[List[int]]
        let py_draws = PyList::new_bound(py, past_draws.iter().map(|row| PyList::new_bound(py, row)));

        // Python 함수 호출
        let result = predictor.getattr("predict").ok()?.call1((py_draws,)).ok()?;

        // Python List[int] → Rust Vec<u32>
        let result_vec: Vec<u32> = result.extract().ok()?;
        Some(result_vec)
    })
}

#[get("/api/lotto/generate")]
async fn generate_numbers(db: web::Data<Arc<Database>>) -> impl Responder {
    // DB에서 전체 1등 번호를 가져와서 2차원 배열로 변환
    let draws = match db.get_all_winning_numbers().await {
        Ok(numbers) => numbers
            .iter()
            .map(|n| vec![n.num1, n.num2, n.num3, n.num4, n.num5, n.num6])
            .collect::<Vec<_>>(),
        Err(e) => {
            eprintln!("DB 데이터 조회 실패: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    // Python predictor로 예측
    let predicted = match call_predictor_pyo3(draws).await {
        Some(nums) => nums,
        None => {
            eprintln!("Python predictor 호출 실패");
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().json(GeneratedNumbers {
        numbers: predicted
    })
}

// 헬스 체크 핸들러 추가
#[get("/healthcheck")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // DB 초기화
    let db = Arc::new(match Database::new().await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("데이터베이스 초기화 실패: {}", e);
            return Ok(());
        }
    });

    // 초기 데이터 로드
    let db_clone = db.clone();
    tokio::spawn(async move {
        update_database(db_clone).await;
    });

    // 주기적 업데이트 스케줄러 시작
    let db_clone = db.clone();
    tokio::spawn(async move {
        schedule_updates(db_clone).await;
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
            )
            // 헬스 체크 서비스 추가
            .service(health_check)
            .service(get_last_winning)
            .service(get_winning_stores)
            .service(get_db_chart)
            .service(generate_numbers)
            .service(
                Files::new("/", "ui/build")
                    .index_file("index.html")
                    .default_handler(|req: ServiceRequest| {
                        let (http_req, _payload) = req.into_parts();
                        async {
                            let response = NamedFile::open("ui/build/index.html")?
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

// 최신 회차 조회 함수
async fn get_latest_round(client: &reqwest::Client) -> Option<u32> {
    let main_page = client
        .get("https://www.dhlottery.co.kr/gameResult.do?method=byWin")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .send()
        .await
        .ok()?;

    let html_content = main_page.text().await.ok()?;
    let document = Html::parse_document(&html_content);
    let selector = Selector::parse("#dwrNoList option:first-child").unwrap();
    
    document.select(&selector).next().map(|element| {
        element.text().collect::<String>()
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<u32>()
            .unwrap_or(0)
    })
}
