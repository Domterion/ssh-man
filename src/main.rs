use std::{env, path::PathBuf};

use crate::config::Config;

mod config;

fn main() {
    let name = env::args().nth(1);

    let config = Config::new(PathBuf::from("config.toml"));

    if let None = name {
        println!("Please pass the name of a server:\n+ {}", config);
        return;
    }

    let server = config.servers.get(&name.unwrap());

    if let Some(s) = server {
        println!("{}", s);
    } else {
        println!("Invalid server name, valid servers:\n+ {}", config);
    }
}
