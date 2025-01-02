use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub use config::*;
pub use error::*;

mod config;
mod error;
mod handlers {
    pub mod sse;
}

pub fn create_app(config: config::Config) -> Router {
    Router::new()
        .route("/", get(handlers::sse::handler))
        .with_state(config)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        )
}
