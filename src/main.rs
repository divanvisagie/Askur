use std::{
    env,
    sync::{Arc, Mutex},
};

use actix_web::{App, HttpResponse, HttpServer, Responder};
use paperclip::actix::{
    api_v2_operation,
    web::{self},
    Apiv2Schema, OpenApiExt,
};
use serde::{Deserialize, Serialize};

#[api_v2_operation]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("All is good")
}

#[derive(Clone, Debug, Serialize, Deserialize, Apiv2Schema)]
struct Capability {
    name: String,
    description: String,
    url: String,
}

#[derive(Clone)]
struct Registry {
    capabilities: Vec<Capability>,
}

impl Registry {
    fn new() -> Self {
        Registry {
            capabilities: Vec::new(),
        }
    }

    fn register(&mut self, capability: Capability) {
        self.capabilities.push(capability);
    }

    #[allow(dead_code)]
    fn get(&self, name: String) -> Option<Capability> {
        for capability in &self.capabilities {
            if capability.name == name {
                return Some(capability.clone());
            }
        }
        None
    }
}

#[api_v2_operation(
    tags("Capabilities"),
    summary = "Register a new capability",
    description = "Register a new capability with the service registry"
)]
fn register_capability(
    cap: web::Json<Capability>,
    registry: web::Data<Arc<Mutex<Registry>>>,
) -> impl Responder {
    let capability = cap.into_inner();
    log::info!("Registering capability: {:?}", capability.clone());
    registry.lock().unwrap().register(capability.clone());

    HttpResponse::Ok().json(capability)
}

#[api_v2_operation(
    tags("Capabilities"),
    summary = "List all capabilities",
    description = "List all capabilities registered with the service registry"
)]
fn list_capabilities(registry: web::Data<Arc<Mutex<Registry>>>) -> impl Responder {
    let registry = registry.lock().unwrap();
    HttpResponse::Ok().json(registry.capabilities.clone())
}

#[api_v2_operation]
fn get_capability(
    name: web::Path<String>,
    registry: web::Data<Arc<Mutex<Registry>>>,
) -> impl Responder {
    let registry = registry.lock().unwrap();
    match registry.get(name.into_inner()) {
        Some(capability) => HttpResponse::Ok().json(capability),
        None => HttpResponse::NotFound().body("Capability not found"),
    }
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

    let registry = Arc::new(Mutex::new(Registry::new()));

    let _h = HttpServer::new(move || {
        App::new()
            .wrap_api()
            .app_data(web::Data::new(registry.clone()))
            .service(web::resource("/health").route(web::get().to(health)))
            .service(
                web::resource("/capabilities")
                    .route(web::get().to(list_capabilities))
                    .route(web::post().to(register_capability)),
            )
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
