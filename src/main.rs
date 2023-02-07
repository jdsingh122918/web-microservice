use std::net::TcpListener;

use web_microservice::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind port");
    run(listener)?.await
}
