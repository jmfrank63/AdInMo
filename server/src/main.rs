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

    database::initialize_root_db_pool()
        .await
        .expect("Failed to initialize root database pool");
    database::initialize_db_pool()
        .await
        .expect("Failed to initialize database pool");

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

#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::{
        http::{self, header},
        test,
    };

    #[actix_web::test]
    async fn test_unprotected_run_endpoint() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppConfig {
                    service_url: "http://127.0.0.1:5500".to_string(),
                }))
                .app_data(web::Data::new(Client::new()))
                .route("/run", web::get().to(run_endpoint)),
        )
        .await;
        let req = test::TestRequest::get().uri("/run").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_protected_endpoint_authorized() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppConfig {
                    service_url: "http://127.0.0.1:5500".to_string(),
                }))
                .app_data(web::Data::new(Client::new()))
                .route("/run", web::get().to(run_endpoint))
                .service(
                    web::scope("/api")
                        .wrap(auth::BasicAuth)
                        .route("/create", web::post().to(create_handler))
                        .route("/read/{id}", web::get().to(read_handler))
                        .route("/update", web::put().to(update_handler))
                        .route("/delete/{id}", web::delete().to(delete_handler)),
                ),
        )
        .await;
        database::initialize_root_db_pool()
            .await
            .expect("Failed to initialize root database pool");
        let req = test::TestRequest::post()
            .uri("/api/create")
            .insert_header((header::AUTHORIZATION, "Basic aHR0cGJpbi11c2VyOmFhYmI="))
            .set_json("test")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), http::StatusCode::UNAUTHORIZED);
    }

    #[actix_web::test]
    async fn test_protected_endpoint_unauthorized() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppConfig {
                    service_url: "http://127.0.0.1:5500".to_string(),
                }))
                .app_data(web::Data::new(Client::new()))
                .route("/run", web::get().to(run_endpoint))
                .service(
                    web::scope("/api")
                        .wrap(auth::BasicAuth)
                        .route("/create", web::post().to(create_handler))
                        .route("/read/{id}", web::get().to(read_handler))
                        .route("/update", web::put().to(update_handler))
                        .route("/delete/{id}", web::delete().to(delete_handler)),
                ),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/api/create")
            .insert_header((header::AUTHORIZATION, ""))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
    }
}
