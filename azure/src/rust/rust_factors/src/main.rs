use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;
use warp::{Filter};
use serde_json::{Value, Map, json};


#[tokio::main]
async fn main() {

    fn convert(n: &String) -> i64 {
        let number_str = n.to_string();
        let number_int: i64 = number_str.parse().unwrap();
        return number_int;
    }

    fn factors(n: i64) -> Vec<i64> {
        let mut vec = Vec::with_capacity(100);
        let mut i: i64 = 1;
        while i <= f64::sqrt(n as f64) as i64 {
            if n % i == 0 {
                vec.push(i);
                if n / i != i {
                    vec.push(n / i);
                }    
            }
            i = i + 1;
        }

        vec.sort();
        return vec;
    }

    let example1 = warp::get()
        .and(warp::path("api"))
        .and(warp::path("factors"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("n") {
            Some(n) => {
                let integer_number = convert(n);
                let res_factors = factors(integer_number);

                let mut map = Map::new();
                let mut inner_map = Map::new();
                inner_map.insert("test".to_string(), serde_json::Value::String("cpu test".to_string()));
                inner_map.insert("N".to_string(), serde_json::Value::String(integer_number.to_string()));
                let v = json!(res_factors);
                inner_map.insert("result".to_string(), v);
                
                map.insert("success".to_string(), Value::Bool(true));
                map.insert("payload".to_string(), serde_json::Value::Object(inner_map));
                warp::reply::json(&map)
            },
            None => {
                let integer_number: i64 = 2688834647444046;
                let res_factors = factors(integer_number);

                let mut map = Map::new();
                let mut inner_map = Map::new();
                inner_map.insert("test".to_string(), serde_json::Value::String("cpu test".to_string()));
                inner_map.insert("N".to_string(), serde_json::Value::String(integer_number.to_string()));
                let v = json!(res_factors);
                inner_map.insert("result".to_string(), v);
                
                map.insert("success".to_string(), Value::Bool(true));
                map.insert("payload".to_string(), serde_json::Value::Object(inner_map));
                warp::reply::json(&map)
            },
        });

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(example1).run((Ipv4Addr::LOCALHOST, port)).await
}