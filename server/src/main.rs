use std::env;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use reqwest::Client;

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
    let server_addr_port = env::var("SERVER_ADDR_PORT").unwrap_or("0.0.0.0:3300".to_string());
    let service_addr_port = env::var("SERVICE_ADDR_PORT").unwrap_or("0.0.0.0:5500".to_string());
    let app_config = web::Data::new(AppConfig {
        service_url: format!(
            "http://service:{}",
            service_addr_port.split(':').nth(1).unwrap_or("5500")
        ),
    });
    let client = web::Data::new(Client::new());

    println!("Actix server running on http://{}", server_addr_port);
    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .app_data(app_config.clone())
            .route("/run", web::get().to(run_endpoint)) // Updated to GET
    })
    .bind(server_addr_port)?
    .run()
    .await
}
