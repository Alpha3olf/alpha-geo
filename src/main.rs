use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::fs;
use std::io::Read;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    if req.method() == hyper::Method::GET{
        if req.uri().path() == "/data.zip" {
            let mut file = fs::File::open("data.zip").unwrap();
            let mut contents: Vec<u8> = Vec::new();
            file.read_to_end(&mut contents).unwrap();
            let response = Response::builder()
                .header("Content-Type", "application/octet-stream")
                .body(Body::from(contents))
                .unwrap();
            Ok(response)
        }
        else {
            Ok(Response::new("Hello, World".into()))
        }

    } else {
        let response = Response::builder()
            .status(404)
            .body(Body::from("Not Found"))
            .unwrap();
        Ok(response)
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request) )
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}