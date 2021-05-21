extern crate clap;
extern crate serde;
extern crate serde_derive;
extern crate serde_yaml;
extern crate xdg;

mod app;
mod config;

use crate::config::Configuration;
use app::Runtime;

fn main() {
    let matches = Runtime.build().get_matches();

    let config = match matches.value_of("config") {
        Some(path) => Configuration::from_file(&path),
        None => Configuration::default(),
    };

    println!("{:?}", config);
}
