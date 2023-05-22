use std::env;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/health")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("All is good")
}

pub async fn start_server() {
    let port = match env::var("PORT") {
        Ok(val) => val,
        Err(_) => "8080".to_string(),
    };
    let port = port.parse::<u16>().expect("PORT must be a number");

    let host = match env::var("HOST") {
        Ok(val) => val,
        Err(_) => "127.0.0.1".to_string(),
    };

    let _h = HttpServer::new(|| {
        App::new().service(hello)
        // .route("/hey", web::get().to(manual_hello))
    })
    .bind((host, port))
    .unwrap()
    .run()
    .await;
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    start_server().await;

    log::info!("Server shutdown");
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[test]
    async fn test_test() {
        assert_eq!(1, 1);
    }
}
