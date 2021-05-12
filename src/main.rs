use std::net::UdpSocket;
use clap::{ Arg, App };

const APP_NAME_SHORT: &str = "BSM";
const APP_NAME_LONG: &str = "Basic Socket Messenger";
const MAX_MSG_SIZE: i16 = 128;

fn main() {
    let args = App::new(format!("{} - {}", APP_NAME_SHORT, APP_NAME_LONG))
        .version("0.1")
        .author("Kevin M. Garner <kevin@kgar.net>")
        .arg(Arg::with_name("host_ip")
             .short("h")
             .long("host")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("remote_ip")
             .short("r")
             .long("remote")
             .required(true)
             .takes_value (true))
        .get_matches();

    let host_ip = args.value_of("host_ip").unwrap();
    let remote_ip = args.value_of("remote_ip").unwrap();

    {
        let socket = UdpSocket::bind(host_ip).expect("Could not bind to IP address");

        println!("{}: Bound to IP {}", APP_NAME_SHORT, host_ip);

        loop {
            println!("{}: Enter message to send. Enter blank message to close", APP_NAME_SHORT);
            let mut input = String::new();
            let _ = std::io::stdin().read_line(&mut input).expect("Error reading input");

            if input.len() > MAX_MSG_SIZE as usize {
                println!("{}: Max message size is {}", APP_NAME_SHORT, MAX_MSG_SIZE);
            } else {
                // remove EOL characters
                if let Some('\n') = input.chars().next_back() {
                    input.pop();
                } else if let Some('\r') = input.chars().next_back() {
                    input.pop();
                }

                // if user wants to quit
                if input == "" {
                    break;
                }
                print!("{}: sending...", APP_NAME_SHORT);
                socket.send_to(input.as_bytes(), &remote_ip).expect("Error sending message");
                println!(" sent! Awaiting response");
                let mut buf = [0; MAX_MSG_SIZE as usize];
                let (_, source) = socket.recv_from(&mut buf).expect("Error in receiving message");
                let buffer_string = String::from_utf8_lossy(&buf);
                println!("{}: Message from {}", APP_NAME_SHORT, source);
                println!("{}", buffer_string);
            }
        }
    } // socket closed
    println!("Connection to {} closed", remote_ip);
}
