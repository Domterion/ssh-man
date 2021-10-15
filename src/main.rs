use std::{env, path::PathBuf};

use crate::config::Config;

mod config;

fn main() {
    let name = env::args().nth(1);

    let config = Config::new(PathBuf::from("config.toml"));

    if name.is_none() {
        println!("Please pass the name of a server:\n{}", config);
        return;
    }

    let server = config.servers.get(&name.clone().unwrap());

    if let Some(s) = server {
        let mut cmd = format!("ssh {}@{} ", s.username, s.ip);

        if let Some(key_file) = &s.key_file {
            let opt = format!("-i {}", key_file.to_str().unwrap());
            cmd = format!("{} {}", cmd, opt);
        }

        println!(
            "To connect to your {} server run:\n\t{}",
            name.unwrap(),
            cmd
        );

        if let Some(password) = &s.password {
            println!("\tUse password: {}", password);
        }
    } else {
        println!("Invalid server name, valid servers:\n{}", config);
    }
}
