//! Contains the bits to start the server
//! # Example
//!
//! ```ignore
//! use std::sync::Arc;
//!
//! use config::Config;
//! use server;
//!
//!
//! #[actix_rt::main]
//! async fn main() {
//!     let state = Arc::new(State {
//!         config: Config
//!     });
//!
//!     server::start(state).await;
//! }

use std::sync::Arc;

use actix_web::{
    guard,
    middleware::{Compress, Logger},
    web, App, HttpServer,
};

use config::Config;

use crate::parser;

/// State to be passed to each route
#[derive(Debug, Clone)]
pub struct State {
    /// Config that maybe useful to certain endpoints
    pub config: Config,
}

/// Sets the state, middleware, and routes and finally starts the server
pub async fn start(state: Arc<State>) -> std::io::Result<()> {
    let addr = state.config.addr.unwrap_or_else(|| {
        error!("Address not set in config. Please make sure that both \"PORT\" and \"HOST\" are set correctly. Panicking..."); 
        panic!("Adress not set. PORT = {}, HOST = {}", state.config.port, state.config.host);
    });

    let status = HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(Compress::default())
            .wrap(Logger::default())
            .configure(add_routes)
    })
    .bind(addr)?
    .run()
    .await;

    status
}

/// Configures the routes
fn add_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("api").route(
            "parse",
            web::post()
                // this guard is in place to ensure that they send a csv as the POST body
                .guard(guard::Header("content-type", "text/csv"))
                .to(parser::parse_csv),
        ),
    );
}
