#[derive(Debug)]
pub enum Error {
    ParseSocketAddr(core::net::AddrParseError),
    ServeGrpcService(tonic::transport::Error),
}
