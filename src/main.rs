extern crate clap;
extern crate serde;
extern crate serde_derive;
extern crate serde_yaml;
extern crate xdg;

mod action;
mod app;
mod config;
mod core;
mod errors;
mod worker;

use crate::core::Core;
use app::Application;
use config::Configuration;

fn main() {
    let matches = Application.build().get_matches();

    let config = match matches.value_of("config") {
        Some(path) => Configuration::from_file(&path),
        None => Configuration::default(),
    };

    println!("{:?}", matches);

    Core::new().from_config(config).with_flags(&matches).run();
}
