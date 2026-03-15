use axum::Router;

pub mod binance;
pub mod database;

pub async fn setup_endpoints(router:Router) -> Router {
    let pool = database::init_database().await;

    let router = binance::setup_endpoints(router);
    let router = database::setup_endpoints(router, pool);
    router
}