#[derive(Debug, Clone)]
pub struct Config {
    pub grpc_addr: String,
}

impl Config {
    pub fn new(grpc_addr: &str) -> Self {
        Self {
            grpc_addr: grpc_addr.into(),
        }
    }
}
