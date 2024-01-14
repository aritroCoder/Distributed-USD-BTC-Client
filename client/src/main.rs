use std::net::TcpStream;
use std::{thread::sleep, time::Duration};
use std::fs::OpenOptions;
use std::io::{Read, Write};

use url::Url;
use tungstenite::{connect, Message};
use serde_json::from_str;
use clap::Parser;
use uuid::Uuid;

use data_models::{Args, SocketResponse};

pub mod data_models;
pub mod utils;

const SOCK_ADDR: &str = "wss://ws-api.binance.com:443/ws-api/v3";

fn cache(argv: Args){
    let (mut socket, _response) = connect(Url::parse(SOCK_ADDR).unwrap()).expect("Can't connect");
    let mut ex_rates: Vec<f64> = vec![];
    for _i in 1..=argv.times {
        let id = Uuid::new_v4();
        let response = socket.write_message(Message::Text(r#"{
            "id": ""#.to_string() + &id.to_string() + r#"",
            "method": "ticker.price",
            "params": {
                "symbol": "BTCUSDC"
            }
        }"#.into()));
        if let Err(e) = response {
            println!("Error sending message: {}", e);
        }
        let msg = socket.read_message().expect("Error reading message");
        let response_parsed = from_str::<SocketResponse>(&msg.to_string());
        match response_parsed {
            Ok(parsed) => {
                let ex_rate = parsed.result.price.parse::<f64>().unwrap_or_default();
                ex_rates.push(ex_rate);
            },
            Err(e) => {
                println!("Error parsing message: {}", e);
            }
        }
        sleep(Duration::from_secs(1));
    }
    let avg_price = ex_rates.iter().sum::<f64>() / ex_rates.len() as f64;
    ex_rates.push(avg_price);
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to parent");
    stream.write_all(avg_price.to_be_bytes().as_ref()).expect("Failed to send message");
    let close_response = socket.close(None);
    if let Err(e) = close_response {
        println!("Error closing the socket: {}", e);
    }
}

fn read(){
    let mut ex_rates: Vec<f64> = vec![];
    let mut file = OpenOptions::new()
        .read(true)
        .open("rates.txt")
        .expect("Error opening file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    for line in contents.lines() {
        let ex_rate = line.parse::<f64>().unwrap_or_default();
        ex_rates.push(ex_rate);
    }
    for i in ex_rates{
        println!("{}", i);
    }
}

fn main() {
    let argv = Args::parse();
    // start only when the start time is reached
    let now = chrono::Utc::now().timestamp();
    if now < argv.start {
        let sleep_time = argv.start - now;
        sleep(Duration::from_secs(sleep_time as u64));
    }
    // println!("Starting the client at {}", argv.start);
    match argv.mode {
        ref mode if mode == "cache" => cache(argv),
        ref mode if mode == "read" => read(),
        _ => println!("Invalid mode. Please use either 'cache' or 'read'"),
    }
}