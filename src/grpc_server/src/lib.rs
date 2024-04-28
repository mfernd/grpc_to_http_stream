use hello_world::{greeter_server::Greeter, HelloReply, HelloRequest};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use tracing::info;

mod error;
pub use error::*;
pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    type SayHelloStream = ReceiverStream<Result<HelloReply, Status>>;

    #[tracing::instrument]
    async fn say_hello(
        &self,
        _: Request<HelloRequest>,
    ) -> Result<Response<Self::SayHelloStream>, Status> {
        info!("beginning stream");
        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            loop {
                let reply = create_hello_reply();
                let _ = tx.send(Ok(reply)).await;
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

fn create_hello_reply() -> HelloReply {
    HelloReply {
        timestamp: chrono::Utc::now().timestamp_millis(),
    }
}
