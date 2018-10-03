
extern crate clap;
extern crate regex;
//extern crate env_logger;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use regex::Regex;
use std::process;
use clap::{Arg, App, SubCommand};
use std::net::{IpAddr, TcpStream,Ipv4Addr, Ipv6Addr};


enum PORTS {
    StartPort,
    EndPort,
    TargetPort
}

fn check_host(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b").unwrap();
    }
    RE.is_match(text)
//    Ok(())
}

fn main() {

    let matches = App::new("Network scan demo")
        .version("0.1")
        .author("Mour")
        .about("Just for rust learning ")
        .arg(Arg::with_name("host")
            .short("h")
            .long("host")
            .value_name("HOST")
            .help("Target Host Name or IP")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("Target Port")
            .multiple(true)
            .takes_value(true)
            .required(true))
        .get_matches();

    let _host = matches.value_of("host").unwrap();
    let _sports = matches.value_of("port").unwrap();

//    let _ports: i32 = _sports.parse().unwrap();
//    let _ports= _sports.parse::<i32>().unwrap();

    let _ports: Vec<i32> = matches.values_of("port").unwrap()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    if check_host(_host) {
        println!("Scan Task"  );
        println!("\thost={}", _host);
        for _port in _ports {
            match _port {
                // `...` express range, `e @` binding value
                e @ 1 ... 65535 => println!("\tport={}",e),
                _ => {},
            }
        }
    }

}
