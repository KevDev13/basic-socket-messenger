use std::net::UdpSocket;
use clap::{ Arg, App };

const APP_NAME_SHORT: &str = "BSM";
const APP_NAME_LONG: &str = "Basic Socket Messenger";

fn main() {
    let args = App::new(format!("{} - {}", APP_NAME_SHORT, APP_NAME_LONG))
        .version("0.1")
        .author("Kevin M. Garner <kevin@kgar.net>")
        .arg(Arg::with_name("ip")
             .short("i")
             .long("ip")
             .required(true)
             .takes_value(true))
        .get_matches();

    let ip_address = args.value_of("ip").unwrap();

    let mut socket = UdpSocket::bind(ip_address).expect("Could not bind to IP address");

    println!("{}: Bound to IP {}", APP_NAME_SHORT, ip_address);
    println!("{}: Enter message to send. Enter blank message to close", APP_NAME_SHORT);
    let mut input = String::new();
    let bytes = std::io::stdin().read_line(&mut input).expect("Error reading input");
    
}
