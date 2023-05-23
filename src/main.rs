use std::env;

use actix_web::{App, HttpResponse, HttpServer, Responder};
use paperclip::actix::{
    api_v2_operation,
    web::{self},
    OpenApiExt,
};

#[api_v2_operation]
async fn health() -> impl Responder {
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
        App::new()
            .wrap_api()
            .service(web::resource("/health").route(web::get().to(health)))
            .with_json_spec_at("/api/spec")
            .with_swagger_ui_at("/swag")
            .build()
    })
    .bind((host, port))
    .unwrap()
    .run()
    .await;
}

#[actix_web::main]
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
    async fn test_health_endpoint() {
        let mut app =
            test::init_service(App::new().service(web::resource("/health").to(health))).await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }
}
