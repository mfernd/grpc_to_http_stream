use axum_server::create_app;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), axum_server::Error> {
    tracing_subscriber::fmt::init();

    let host = std::env::var("APP_HOST").unwrap_or("127.0.0.1".into());
    let port = std::env::var("APP_PORT").unwrap_or("3000".into());
    let addr: SocketAddr = get_addr(host, port)?;

    let grpc_addr = std::env::var("GRPC_URI").unwrap_or("localhost:50051".into());
    let config = axum_server::Config::new(&grpc_addr);

    let app = create_app(config);

    let listener = TcpListener::bind(addr)
        .await
        .map_err(axum_server::Error::TokioListenerBind)?;

    info!("starting http server on http://{}/", addr);
    axum::serve(listener, app)
        .await
        .map_err(axum_server::Error::ServeHttpService)?;

    Ok(())
}

fn get_addr(host: String, port: String) -> Result<SocketAddr, axum_server::Error> {
    format!("{}:{}", host, port)
        .parse()
        .map_err(axum_server::Error::ParseSocketAddr)
}
