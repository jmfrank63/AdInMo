use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle_request(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Request received, success")))
}

#[tokio::main]
async fn main() {
    // Define the socket address for the server (localhost:5000)
    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));

    // Create a service that will handle incoming requests using handle_request function
    let service = make_service_fn(|_| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    // Create and run the server
    let server = Server::bind(&addr).serve(service);

    println!("Hyper server running on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
