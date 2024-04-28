use common::hello_world::greeter_server::GreeterServer;
use grpc_server::MyGreeter;
use std::net::SocketAddr;
use tonic::transport::Server;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), grpc_server::Error> {
    tracing_subscriber::fmt::init();

    let host = std::env::var("APP_HOST").unwrap_or("127.0.0.1".into());
    let port = std::env::var("APP_PORT").unwrap_or("50051".into());
    let addr: SocketAddr = get_addr(host, port)?;

    let svc = GreeterServer::new(MyGreeter);

    info!("starting grpc server on {}", addr);
    Server::builder()
        .add_service(svc)
        .serve(addr)
        .await
        .map_err(grpc_server::Error::ServeGrpcService)?;

    Ok(())
}

fn get_addr(host: String, port: String) -> Result<SocketAddr, grpc_server::Error> {
    format!("{}:{}", host, port)
        .parse()
        .map_err(grpc_server::Error::ParseSocketAddr)
}
