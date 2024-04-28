use crate::config;
use axum::{
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
};
use common::hello_world::{greeter_client::GreeterClient, HelloReply, HelloRequest};
use futures::Stream;
use serde::Serialize;
use std::convert::Infallible;
use tokio_stream::StreamExt;
use tracing::{error, info};

#[tracing::instrument]
pub async fn handler(
    State(config): State<config::Config>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    info!("beginning stream");

    let mut client = GreeterClient::connect(format!("http://{}", config.grpc_addr))
        .await
        .unwrap();

    let request = tonic::Request::new(HelloRequest {});
    let mut response = client.say_hello(request).await.unwrap().into_inner();

    let stream = async_stream::stream! {
        while let Some(Ok(exec_response)) = response.next().await {
            match Event::default().json_data::<ResponseJson>(exec_response.into()) {
                Ok(json) => yield Ok(json),
                Err(e) => error!("SSE error: {:?}", e),
            }
        }
    };

    Sse::new(stream).keep_alive(KeepAlive::default())
}

#[derive(Debug, Serialize)]
struct ResponseJson {
    timestamp: i64,
}

impl From<HelloReply> for ResponseJson {
    fn from(value: HelloReply) -> Self {
        Self {
            timestamp: value.timestamp,
        }
    }
}
