#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug)]
struct Server {
    ip: Option<String>,
    port: Option<i32>,
}

#[derive(Deserialize, Debug)]
struct Conf {
    server: Server,
}

fn main() {
    let file_path = "res/server.toml";
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception: {}", file_path, e),
    };

    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e),
    };

    let conf: Conf = toml::from_str(&str_val).unwrap();
    println!("server info --- {:?}", conf.server);
}
