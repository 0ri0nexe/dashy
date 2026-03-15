use log::{info};
use axum::Router;

pub mod binance;
pub mod database;

pub async fn setup_endpoints(router:Router) -> Router {
    let pool = database::init_database().await;

    info!("Endpoint initialization");
    let router = binance::setup_endpoints(router);
    let router = database::setup_endpoints(router, pool);
    
    router
}