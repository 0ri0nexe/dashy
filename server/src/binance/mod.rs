use serde::{Deserialize, Serialize};
use axum::{Json, Router, routing::get};
use reqwest;

pub enum Symbols {
	BTC,
}

#[derive(Serialize)]
pub struct SymbolResponse {
    symbol: String,
	price : f64,
}

#[derive(Serialize, Deserialize)]
struct BinanceResponse {
    symbol:String,
    price :String,
}

pub async fn get_price(symbol:Symbols) -> Json<SymbolResponse> {
	let to_get: String;

	match symbol {
		Symbols::BTC => to_get = String::from("BTCUSDC")
	}
    
	let response: BinanceResponse = reqwest::get(
        format!(
            "https://api.binance.com/api/v3/ticker/price?symbol={}",
            to_get
        )
    ).await
    .unwrap()
    .json::<BinanceResponse>().await.unwrap();

    let parsed_price = response.price.parse::<f64>().unwrap();

    Json(SymbolResponse {
        symbol: response.symbol,
        price: parsed_price,
    })
}

pub fn setup_endpoints(router:Router) -> Router {
    router.route("/api/price/BTC", get(| | get_price(Symbols::BTC)))
}