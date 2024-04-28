#[derive(Debug)]
pub enum Error {
    ParseSocketAddr(core::net::AddrParseError),
    TokioListenerBind(std::io::Error),
    ServeHttpService(std::io::Error),
}
