use reqwest::{Client, Error, StatusCode};
use serde_json::json;
use tokio::signal;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let server_addr_port = std::env::var("SERVER_ADDR_PORT").expect("SERVER_ADDR_PORT must be set");
    get_run(&server_addr_port).await.unwrap();
    read_unauthorized(&server_addr_port).await.unwrap();
    post_create(&server_addr_port).await.unwrap();
    get_read(&server_addr_port).await.unwrap();
    delete_delete(&server_addr_port).await.unwrap();
    put_update(&server_addr_port).await.unwrap();

    // Wait for CTRL+C signal
    println!("Tests ran sucessfully. Press CTRL+C to exit.");
    signal::ctrl_c().await.expect("Failed to listen for event");
    println!("\nShutting down.");
}

async fn get_run(server_addr_port: &str) -> Result<(), Error> {
    print!("Running GET /run on {server_addr_port}");
    let client = Client::new();
    let response = client.get(format!("http://{server_addr_port}/run")).send().await?;
    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.text().await?.contains('['));
    println!("...OK");
    Ok(())
}

async fn post_create(server_addr_port: &str) -> Result<(), Error> {
    print!("Running POST /api/create");
    let client = Client::new();
    let response = client
        .post(format!("http://{server_addr_port}/api/create"))
        .basic_auth("httpbin-user", Some("aabb"))
        .json(&json!({
  "value": 7,
  "response_body": "{\"args\":{},\"data\":\"{\\\"value\\\":7}\",\"files\":{},\"form\":{},\"headers\":{\"Accept\":\"*/*\",\"Content-Length\":\"11\",\"Content-Type\":\"application/json\",\"Host\":\"httpbin.org\",\"X-Amzn-Trace-Id\":\"Root=1-655883ea-113a250c75de0e3107e65f14\"},\"json\":{\"value\":7},\"origin\":\"194.32.120.197\",\"url\":\"https://httpbin.org/post\"}"
}))
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::CREATED);
    assert!(response.text().await?.contains("Request created"));
    println!("...OK");
    Ok(())
}

async fn get_read(server_addr_port: &str) -> Result<(), Error> {
    print!("Running GET /api/read/5");
    let client = Client::new();
    let response = client
        .get(format!("http://{server_addr_port}/api/read/5"))
        .basic_auth("httpbin-user", Some("aabb"))
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.text().await?.contains("\"id\":5"));
    println!("...OK");
    Ok(())
}

async fn delete_delete(server_addr_port: &str) -> Result<(), Error> {
    print!("Running DELETE /api/delete/3");
    let client = Client::new();
    let response = client
        .delete(format!("http://{server_addr_port}/api/delete/3"))
        .basic_auth("httpbin-user", Some("aabb"))
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.text().await?.contains("Request deleted"));
    println!("...OK");
    Ok(())
}

async fn put_update(server_addr_port: &str) -> Result<(), Error> {
    print!("Running PUT /api/update/2");
    let client = Client::new();
    let response = client
        .put(format!("http://{server_addr_port}/api/update/2"))
        .basic_auth("httpbin-user", Some("aabb"))
        .json(&json!({
  "id": 2,
  "value": 20,
  "response_body": "{\"args\":{},\"data\":\"{\\\"value\\\":20}\",\"files\":{},\"form\":{},\"headers\":{\"Accept\":\"*/*\",\"Content-Length\":\"11\",\"Content-Type\":\"application/json\",\"Host\":\"httpbin.org\",\"X-Amzn-Trace-Id\":\"Root=1-655883ea-113a250c75de0e3107e65f14\"},\"json\":{\"value\":7},\"origin\":\"194.32.120.197\",\"url\":\"https://httpbin.org/post\"}"
  }))
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.text().await?.contains("Request updated"));
    println!("...OK");
    Ok(())
}

async fn read_unauthorized(server_addr_port: &str) -> Result<(), Error> {
    print!("Running unauthorized GET /api/read/5");
    let client = Client::new();
    let response = client
        .get(format!("http://{server_addr_port}/api/read/5"))
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    println!("...OK");
    Ok(())
}
