use std::net::UdpSocket;
use std::env;
use colored::*;

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();
	let port: i32;
	let host: &str;
	if args.len() >= 2 {
		let result: Result<i32, _> = args[1].parse();
		port = match result {
			Ok(n) => n,
			Err(_) => {
				println!("Invalid port provided, using 1235");
				1235
			}
		};
		if args.len() == 3 {
			host = &args[2];
		}
		else {
			host = "127.0.0.1";
		}
	}
	else {
		host = "127.0.0.1";
		port = 1235;
	}
	
	let addr = format!("{}:{}", host, port);
	let socket = UdpSocket::bind(&addr)?;
	println!("Bound to {}", addr);
	let colors = [Color::Red, Color::Yellow, Color::BrightYellow, Color::BrightGreen, Color::Green, Color::Blue, Color::Magenta];
	let mut color_index: usize = 0; 
	let mut buf = [0u8; 32768];
	loop {
		let (amt, _src) = socket.recv_from(&mut buf)?;
		println!("{}", String::from_utf8_lossy(&buf[..amt]).color(colors[color_index]));
		color_index += 1;
		if color_index >= colors.len() {
			color_index = 0;
		}
	}
}
