use std::collections::HashMap;
use rand::Rng;
use std::env;
use std::net::Ipv4Addr;
use warp::{Filter};
use serde_json::{Value, Map};


#[tokio::main]
async fn main() {

    fn convert(n: &String) -> i64 {
        let number_str = n.to_string();
        let number_int: i64 = number_str.parse().unwrap();
        return number_int;
    }

    fn random_table(n: i64) -> Vec<Vec<i64>> {
        let mut vec1 = Vec::with_capacity(n.try_into().unwrap());
        let mut rng = rand::thread_rng();

        let mut i: i64 = 0;
        while i < n {
            vec1.push(Vec::with_capacity(n.try_into().unwrap()));
            
            let mut j: i64 = 0;
            while j < n {
                vec1[i as usize].push(rng.gen_range(0..100));
                j = j + 1;
            }
            i = i + 1;
        }
        
        return vec1;
    }

    fn matrix(n: i64) -> Vec<Vec<i64>> {
        let matrix_a = random_table(n);
        let matrix_b = random_table(n);
        let mut matrix_mult = Vec::with_capacity(n.try_into().unwrap());

        let mut i: usize = 0;
        while i < matrix_a.len() {
            matrix_mult.push(Vec::with_capacity(n.try_into().unwrap()));

            let mut j: usize = 0;
            while j < matrix_b.len() {
                let mut sum = 0;
                let mut k: usize = 0;
                while k  < matrix_a.len() {
                    sum = sum + matrix_a[i][k] * matrix_b[k][j];
                    k = k + 1;
                }
                matrix_mult[i as usize].push(sum);
                j = j + 1;
            }
            i = i + 1;
        }

        return matrix_mult;
    }

    let example1 = warp::get()
        .and(warp::path("api"))
        .and(warp::path("matrix"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("n") {
            Some(n) => {
                let integer_number = convert(n);
                matrix(integer_number);

                let mut map = Map::new();
                let mut inner_map = Map::new();
                inner_map.insert("test".to_string(), serde_json::Value::String("matrix test".to_string()));
                inner_map.insert("N".to_string(), serde_json::Value::String(integer_number.to_string()));
                
                map.insert("success".to_string(), Value::Bool(true));
                map.insert("payload".to_string(), serde_json::Value::Object(inner_map));
                warp::reply::json(&map)
            },
            None => {
                let integer_number: i64 = 100;
                matrix(integer_number);

                let mut map = Map::new();
                let mut inner_map = Map::new();
                inner_map.insert("test".to_string(), serde_json::Value::String("matrix test".to_string()));
                inner_map.insert("N".to_string(), serde_json::Value::String(integer_number.to_string()));
                
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