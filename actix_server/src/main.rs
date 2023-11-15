use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;

// Updated to handle GET requests
async fn run_endpoint(client: web::Data<Client>) -> impl Responder {
    // Construct the URL for the service running on port 5000
    let service_url = "http://service:5000";

    // Send a GET request to the service
    let response = client.get(service_url)
        .send()
        .await;

    match response {
        Ok(_) => HttpResponse::Ok().body("Request forwarded to the service"),
        Err(_) => HttpResponse::InternalServerError().body("Error forwarding the request"),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let client = web::Data::new(Client::new());

    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .route("/run", web::get().to(run_endpoint)) // Updated to GET
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
