use actix_web::{web, App, HttpResponse, HttpServer, Responder, Error};
use actix_web::web::Bytes;
use actix_cors::Cors;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time;
use chrono::Local;
use serde::{Serialize, Deserialize};
use rand::Rng;
use actix_files as actix_files;

// Shared state for connections and message history
struct ServerState {
    total_requests: Arc<Mutex<u64>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct SystemLoad {
    one_min: f64,
    five_min: f64,
    fifteen_min: f64,
}

async fn stats(state: web::Data<ServerState>) -> impl Responder {
    let mut total_requests = state.total_requests.lock().unwrap();
    *total_requests += 1;

    HttpResponse::Ok()
        .append_header(("content-type", "text/event-stream"))
        .append_header(("cache-control", "no-cache"))
        .append_header(("connection", "keep-alive"))
        .streaming(async_stream::stream! {
            let mut rng = rand::thread_rng();
            loop {
                // Generate random system load values between 0 and 1
                let load = SystemLoad {
                    one_min: rng.gen::<f64>(),
                    five_min: rng.gen::<f64>(),
                    fifteen_min: rng.gen::<f64>(),
                };

                // Current time
                let current_time = Local::now().timestamp_millis();

                // Yield events
                let uptime_event = format!(
                    "event: uptime\ndata: {}\n\n", 
                    serde_json::to_string(&load).unwrap()
                );
                yield Ok::<Bytes, Error>(Bytes::from(uptime_event));

                let time_event = format!(
                    "event: time\ndata: {}\n\n", 
                    current_time
                );
                yield Ok::<Bytes, Error>(Bytes::from(time_event));

                // Sleep for a bit before next event
                time::sleep(Duration::from_secs(1)).await;
            }
        })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting SSE server at http://127.0.0.1:8000");
    
    let server_state = web::Data::new(ServerState {
        total_requests: Arc::new(Mutex::new(0)),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(server_state.clone())
            .route("/stats", web::get().to(stats))
            .service(actix_files::Files::new("/", "./").index_file("index.html"))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
