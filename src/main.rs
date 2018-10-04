//extern crate pnet;
//working in windows has some problem
extern crate clap;
extern crate regex;
//extern crate env_logger;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use regex::Regex;
use std::process;
use std::thread;
use clap::{Arg, App, SubCommand};
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::net::{IpAddr, Ipv4Addr};

enum PORTS {
    StartPort,
    EndPort,
    TargetPort
}

enum SCANTYPE {
    TCP,
    PING
}

fn scan_port(host: &str, port: u16) -> bool {
    match TcpStream::connect((host, port)) {
        Ok(_) => {
            true
        },
        Err(_) => false,
    }
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
            )
        .get_matches();

    let _host = matches.value_of("host").unwrap();
//    let _sports = matches.value_of("port").unwrap();

//    let _ports: i32 = _sports.parse().unwrap();
//    let _ports= _sports.parse::<i32>().unwrap();

    let _ports: Vec<u16> = matches.values_of("port").unwrap()
        .map(|s| s.parse::<u16>().unwrap())
        .collect();

    let mut openports = Vec::new();

    if check_host(_host) {
        println!("Scan Task"  );
        println!("\thost={}", _host);
        print!("\tport=");
        for _port in _ports {
            match _port {
                // `...` express range, `e @` binding value
                e @ 1 ... 65535 => {
                    let res = scan_port(_host ,_port);
                    print!("{} ",e);
                    if res{
                        openports.push(_port);
                    }
                },
                _ => {},
            }
        }

        print!("\n\tOpening:");
        if !openports.is_empty() {
            for _open in openports {
                print!("{}",_open);
            }
        }else{
            print!("None");
        }
    }else {
        error!("Your host is not ip format or it not a regular host");
    }
}
