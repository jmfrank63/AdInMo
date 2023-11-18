mod auth;
mod crud;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use env_logger::Env;
use reqwest::Client;
use std::env;

use crate::crud::{create_handler, delete_handler, read_handler, update_handler};

struct AppConfig {
    service_url: String,
}

async fn run_endpoint(
    client: web::Data<Client>,
    app_config: web::Data<AppConfig>,
) -> impl Responder {
    // Use the stored service URL
    let service_url = &app_config.service_url;

    // Send a GET request to the service and await the response
    match client.get(service_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(body) => HttpResponse::Ok().body(body),
                    Err(_) => {
                        HttpResponse::InternalServerError().body("Failed to read response body")
                    }
                }
            } else {
                HttpResponse::InternalServerError().body("Service responded with an error")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error forwarding the request"),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let env = Env::default()
        .filter_or("APP_LOG_LEVEL", "info") // Default log level
        .write_style_or("APP_LOG_STYLE", "auto");
    env_logger::init_from_env(env);

    database::initialize_db_pool()
        .await
        .expect("Failed to initialize database pool");

    database::initialize_root_db_pool()
        .await
        .expect("Failed to initialize root database pool");

    let server_addr_port = env::var("SERVER_ADDR_PORT").unwrap_or("0.0.0.0:3300".to_string());
    let service_addr_port = env::var("SERVICE_ADDR_PORT").unwrap_or("0.0.0.0:5500".to_string());
    let app_config = web::Data::new(AppConfig {
        service_url: format!(
            "http://service:{}",
            service_addr_port.split(':').nth(1).unwrap_or("5500")
        ),
    });
    let client = web::Data::new(Client::new());

    log::info!("Actix server running on http://{}", server_addr_port);
    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .app_data(app_config.clone())
            .route("/run", web::get().to(run_endpoint))
            .service(
                web::scope("/api")
                    .wrap(auth::BasicAuth)
                    .route("/create", web::post().to(create_handler))
                    .route("/read/{id}", web::get().to(read_handler))
                    .route("/update", web::put().to(update_handler))
                    .route("/delete/{id}", web::delete().to(delete_handler)),
            )
    })
    .bind(server_addr_port)?
    .run()
    .await
}
