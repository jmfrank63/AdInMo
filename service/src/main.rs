use dotenv::dotenv;
use env_logger::Env;
use futures::future::join_all;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;

const HTTP_BIN_URL: &str = "https://httpbin.org/post";

async fn handle_request(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    let client = reqwest::Client::new();
    let mut requests = Vec::new();

    for _ in 0..30 {
        let random_value = rand::random::<u8>() % 11; // Generates a number between 0 and 10
        let client_clone = client.clone();
        let request = async move {
            client_clone
                .post(HTTP_BIN_URL)
                .json(&json!({"value": random_value}))
                .send()
                .await
        };
        requests.push(request);
    }

    let mut freq_map = HashMap::new();
    let responses = join_all(requests).await;
    for response in responses.into_iter().flatten() {
        let response_body = match response.json::<Value>().await {
            Ok(response_body) => response_body,
            Err(e) => {
                log::error!("Error parsing response body: {}", e);
                continue;
            }
        };
        let generated_value = match response_body["json"]["value"].as_u64() {
            Some(generated_value) => generated_value,
            None => {
                log::error!("Error parsing generated value");
                continue;
            }
        };
        *freq_map.entry(generated_value).or_insert(0) += 1;
        if let Err(e) =
            database::insert_into_database(generated_value as i32, &response_body.to_string()).await
        {
            log::error!("Error inserting into database: {}", e);
        }
    }

    let frequent_numbers = calculate_most_frequent_numbers(freq_map);

    let response_body = serde_json::to_string(&frequent_numbers).unwrap_or_else(|e| {
        log::warn!("Error parsing json response body: {e}");
        "[]".to_string()
    });

    Ok(Response::new(Body::from(response_body)))
}

fn calculate_most_frequent_numbers(freq_map: HashMap<u64, i32>) -> Vec<u64> {
    // Process freq_map to find and sort the most frequent numbers
    let mut freq_vec: Vec<_> = freq_map.into_iter().collect();
    freq_vec.sort_by(|a, b| a.0.cmp(&b.0));

    // Filter out numbers that appear only once and return the vector
    freq_vec
        .into_iter()
        .filter(|&(_, count)| count > 1)
        .map(|(val, _)| val)
        .collect()
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let env = Env::default()
        .filter_or("APP_LOG_LEVEL", "info") // Default log level
        .write_style_or("APP_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    database::initialize_db_pool()
        .await
        .expect("Failed to initialize database pool");

    // Create a service that will handle incoming requests using handle_request function
    let service =
        make_service_fn(move |_| async move { Ok::<_, Infallible>(service_fn(handle_request)) });

    // Create and run the server
    let service_addr_port =
        std::env::var("SERVICE_ADDR_PORT").unwrap_or("0.0.0.0:5500".to_string());
    let addr = SocketAddr::from_str(&service_addr_port)
        .unwrap_or_else(|e| panic!("Unable to parse socket address {service_addr_port}: {e}"));
    let server = Server::bind(&addr).serve(service);

    log::info!("Hyper server running on http://{}", addr);

    if let Err(e) = server.await {
        log::error!("server error: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_occurrences() {
        let mut freq_map = HashMap::new();
        freq_map.insert(1, 1);
        freq_map.insert(2, 1);
        freq_map.insert(3, 1);

        let result = calculate_most_frequent_numbers(freq_map);
        assert!(result.is_empty());
    }

    #[test]
    fn test_multiple_occurrences() {
        let mut freq_map = HashMap::new();
        freq_map.insert(1, 2);
        freq_map.insert(2, 1);
        freq_map.insert(3, 3);

        let result = calculate_most_frequent_numbers(freq_map);
        assert_eq!(result, vec![1, 3]);
    }

    #[test]
    fn test_various_occurrences() {
        let mut freq_map = HashMap::new();
        for &num in &[3, 2, 5, 1, 5, 7, 2, 1] {
            *freq_map.entry(num).or_insert(0) += 1;
        }
        let result = calculate_most_frequent_numbers(freq_map);
        assert_eq!(result, vec![1, 2, 5]);
    }

    #[test]
    fn test_single_repeated_number() {
        let mut freq_map = HashMap::new();
        for &num in &[5, 7, 7] {
            *freq_map.entry(num).or_insert(0) += 1;
        }
        let result = calculate_most_frequent_numbers(freq_map);
        assert_eq!(result, vec![7]);
    }

    #[test]
    fn test_no_occurrences() {
        let freq_map = HashMap::new();

        let result = calculate_most_frequent_numbers(freq_map);
        assert!(result.is_empty());
    }
}
