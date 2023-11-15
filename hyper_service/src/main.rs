use dotenv::dotenv;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;

async fn handle_request(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Request received, success")))
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
