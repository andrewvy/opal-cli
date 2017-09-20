extern crate clap;
extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;
extern crate tls_api;
extern crate tls_api_openssl;

use clap::{Arg, App};

pub mod opal;
pub mod opal_grpc;

fn main() {
    let matches = App::new("opal-cli")
        .version("0.1.0")
        .author("Andrew Vy <andrew@andrewvy.com>")
        .about("cli tool for the opal cryptocurrency")
        .arg(Arg::with_name("COMMAND")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("Command to run"))
        .get_matches();

    let command = matches.value_of("COMMAND").unwrap();
    
    println!("{}", command);
}
