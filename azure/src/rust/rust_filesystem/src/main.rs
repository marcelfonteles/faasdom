use std::collections::HashMap;
use rand::Rng;
use std::env;
use std::net::Ipv4Addr;
use warp::{Filter};
use serde_json::{Value, Map};
use std::fs;
use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() {

    fn convert(n: &String) -> i64 {
        let number_str = n.to_string();
        let number_int: i64 = number_str.parse().unwrap();
        return number_int;
    }

    fn filesystem(n: i64, size: i64) -> std::io::Result<()> {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..900000);

        fs::create_dir_all("/tmp/test")?;
        let path = format!("/tmp/test/{}", random);
        fs::create_dir_all(path)?;

        let mut string: String = "".to_owned();

        let mut i: i64 = 0;
        while i < size {
            string.push_str("A");
            i = i + 1;
        }

        // Write
        i = 0;
        while i < n {
            let path_filename = format!("/tmp/test/{}/{}.txt", random, i);
            let mut file = File::create(path_filename)?;
            file.write_all(string.as_bytes())?;
            i = i + 1;
        }

        // Read
        let mut test: String = "".to_owned();
        i = 0;
        while i < n {
            let path_filename = format!("/tmp/test/{}/{}.txt", random, i);
            let mut file = File::open(path_filename)?;
            // let mut contents = String::new();
            file.read_to_string(&mut test)?;
            i = i + 1;
        }
        println!("{}", test);

        Ok(())
    }

    let example1 = warp::get()
        .and(warp::path("api"))
        .and(warp::path("filesystem"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("n") {
            Some(n) => {
                let integer_number = convert(n);
                let size = 10240;
                let _res = filesystem(integer_number, size);

                let mut map = Map::new();
                let mut inner_map = Map::new();
                inner_map.insert("test".to_string(), serde_json::Value::String("filesystem test".to_string()));
                inner_map.insert("N".to_string(), serde_json::Value::String(integer_number.to_string()));
                inner_map.insert("Size".to_string(), serde_json::Value::String(size.to_string()));
                
                map.insert("success".to_string(), Value::Bool(true));
                map.insert("payload".to_string(), serde_json::Value::Object(inner_map));
                warp::reply::json(&map)
            },
            None => {
                let integer_number: i64 = 10000;
                let size = 10240;
                let _res = filesystem(integer_number, size);

                let mut map = Map::new();
                let mut inner_map = Map::new();
                inner_map.insert("test".to_string(), serde_json::Value::String("filesystem test".to_string()));
                inner_map.insert("N".to_string(), serde_json::Value::String(integer_number.to_string()));
                inner_map.insert("Size".to_string(), serde_json::Value::String(size.to_string()));
                
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