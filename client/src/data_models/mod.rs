use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser,Default,Debug)]
#[clap(author="Aritra Bhaduri", version, about="A simple websocket client to cache and read ETH/USD exchange rates from websocket feeds.")]
pub struct Args {
    #[clap(short, long)]
    pub start: i64,
    #[clap(short, long)]
    pub mode: String,
    #[clap(short, long, default_value_t = 0)]
    pub times: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ExchangeData {
    pub symbol: String,
    pub price: String,
}

#[derive(Serialize, Deserialize)]
pub struct SocketResponse {
    pub id: String,
    pub status: i32,
    pub result: ExchangeData,
}