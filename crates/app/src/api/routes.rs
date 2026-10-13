use besi::axum::Router;
use besi::axum::routing::get;
use besi::{ApiRoutes, ApiState};

use super::handlers;

/// Marker type naming this app's routes: `besi::ApiCommand<AppRoutes>`.
#[derive(Debug)]
pub struct AppRoutes;

impl ApiRoutes for AppRoutes {
    fn routes() -> Router<ApiState> {
        Router::new().nest("/api", api_routes())
    }
}

fn api_routes() -> Router<ApiState> {
    Router::new().route("/hello/{name}", get(handlers::hello::hello))
}