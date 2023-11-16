use dotenv::dotenv;
use futures::future::join_all;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use serde_json::json;
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;

const HTTP_BIN_URL: &str = "https://httpbin.org/post";

async fn handle_request(_: Request<Body>, db_pool: Arc<MySqlPool>) -> Result<Response<Body>, Infallible> {
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

    let frequent_numbers = calculate_most_frequent_numbers(freq_map);

    let response_body =
        serde_json::to_string(&frequent_numbers).unwrap_or_else(|_| "[]".to_string());

    Ok(Response::new(Body::from(response_body)))
}

fn calculate_most_frequent_numbers(freq_map: HashMap<u64, u32>) -> Vec<u64> {
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

    let db_pool = mariadb_database::get_db_pool().await;
    let db_pool = Arc::new(db_pool);

    let service_addr_port =
        std::env::var("SERVICE_ADDR_PORT").unwrap_or("0.0.0.0:5500".to_string());
    let addr = SocketAddr::from_str(&service_addr_port)
        .unwrap_or_else(|_| panic!("Unable to parse socket address {}", service_addr_port));

    // Create a service that will handle incoming requests using handle_request function
    let service = make_service_fn(|_| async { Ok::<_, Infallible>(service_fn(handle_request)) });
    let service = make_service_fn(move |_| {
    let db_pool_clone = db_pool.clone();
    async move {
        Ok::<_, Infallible>(service_fn(move |req| handle_request(req, db_pool_clone.clone())))
    }
});

    // Create and run the server
    let server = Server::bind(&addr).serve(service);

    println!("Hyper server running on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
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
