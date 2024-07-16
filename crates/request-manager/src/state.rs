//! Application state management for the User Request Manager.
//!
//! The `AppState` struct holds shared state information needed across various components
//! of the application, such as HTTP clients and Starknet contract handlers.

use reqwest::Client;

/// Represents the application state shared across various components.
///
/// `AppState` holds instances of `Client` for making HTTP requests.
/// In the future, it is intended to also hold instances of `FactRegistry` and `L1HeadersStore`
/// for interacting with Starknet contracts.
///
/// # Fields
/// * `client` - An instance of `reqwest::Client` for making HTTP requests.
/// * `fact_registry` - (planned) An instance of `FactRegistry` for interacting with the Fact Registry contract on Starknet.
/// * `l1_headers_store` - (planned) An instance of `L1HeadersStore` for interacting with the L1 Headers Store contract on Starknet.
#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    // pub fact_registry: FactRegistry,
    // pub l1_headers_store: L1HeadersStore,
}
