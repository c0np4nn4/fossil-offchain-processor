//! # User Request Manager
//!
//! The User Request Manager is responsible for managing user requests for specific historical
//! Ethereum account storage values and coordinating interactions with backend components.
//!
//! ## Usage
//! Run the application using the following command:
//! ```sh
//! cargo run
//! ```
//!
//! ## Example
//!
//! To request storage value, send a POST request to `/get-storage` endpoint with the appropriate payload:
//! ```sh
//! curl -X POST http://localhost:8000/get-storage -d '{"block_number": 123456, "account_address": "0x...", "slot": "0x...", "storage_keys": ["0x..."]}'
//! ```

use axum::{extract::MatchedPath, http::Request, routing::post, Router};
use reqwest::Client;
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod handlers;
mod state;

use crate::state::AppState;
use handlers::get_storage_value::get_storage_value;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "request_manager=info,tower_http=debug,axum=info,tokio=info".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_state = AppState {
        client: Client::new(),
        // fact_registry: fact_registry_contract, //
        // l1_headers_store: l1_headers_store_contract,
    };

    let app = Router::new()
        .route("/get-storage", post(get_storage_value))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        )
        .with_state(app_state);

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:8000").await {
        Ok(listener) => {
            tracing::info!("Listening on http://{}", listener.local_addr().unwrap());
            listener
        }
        Err(err) => {
            tracing::error!("Failed to bind to address: {:?}", err);
            return;
        }
    };

    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("Server error: {}", e);
    }
}
