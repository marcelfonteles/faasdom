use std::env;
use std::net::Ipv4Addr;
use warp::{Filter};
use serde_json::{Value, Map};


#[tokio::main]
async fn main() {

    let example1 = warp::get()
        .and(warp::path("api"))
        .and(warp::path("latency"))
        .map(|| {
            let mut map = Map::new();
            let mut inner_map = Map::new();
            inner_map.insert("test".to_string(), serde_json::Value::String("latency test".to_string()));
            map.insert("success".to_string(), Value::Bool(true));
            map.insert("payload".to_string(), serde_json::Value::Object(inner_map));
            warp::reply::json(&map)
        });

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(example1).run((Ipv4Addr::LOCALHOST, port)).await
}