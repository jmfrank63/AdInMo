use dotenv::dotenv;
use futures::future::join_all;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;
use serde_json::json;
use std::collections::HashMap;

const HTTP_BIN_URL: &str = "https://httpbin.org/post";

async fn handle_request(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    let client = reqwest::Client::new();
    let mut requests = Vec::new();

    for _ in 0..30 {
        let random_value = rand::random::<u8>() % 11; // Generates a number between 0 and 10
        let client_clone = client.clone();
        let request = async move {
            client_clone.post(HTTP_BIN_URL)
                        .json(&json!({"value": random_value}))
                        .send().await
        };
        requests.push(request);
    }

    let responses = join_all(requests).await;
    let mut freq_map = HashMap::new();

    for response in responses {
        if let Ok(res) = response {
            if let Ok(json) = res.json::<serde_json::Value>().await {
                if let Some(value) = json["json"]["value"].as_u64() {
                    *freq_map.entry(value).or_insert(0) += 1;
                }
            }
        } else {
            // Log error but continue processing
        }
    }

    // Process freq_map to find and sort the most frequent numbers
    let mut freq_vec: Vec<_> = freq_map.into_iter().collect();
    freq_vec.sort_by(|a, b| a.0.cmp(&b.0));

    // Filter out numbers that appear only once
    let frequent_numbers: Vec<_> = freq_vec.into_iter()
                                           .filter(|&(_, count)| count > 1)
                                           .map(|(val, _)| val)
                                           .collect();

    let response_body = serde_json::to_string(&frequent_numbers).unwrap_or_else(|_| "[]".to_string());

    Ok(Response::new(Body::from(response_body)))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Define the socket address for the server (default 0.0.0.0:5500)
    let service_addr_port =
        std::env::var("SERVICE_ADDR_PORT").unwrap_or("0.0.0.0:5500".to_string());
    let addr = SocketAddr::from_str(&service_addr_port)
        .unwrap_or_else(|_| panic!("Unable to parse socket address {}", service_addr_port));

    // Create a service that will handle incoming requests using handle_request function
    let service = make_service_fn(|_| async { Ok::<_, Infallible>(service_fn(handle_request)) });

    // Create and run the server
    let server = Server::bind(&addr).serve(service);

    println!("Hyper server running on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
