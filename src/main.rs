use std::net::UdpSocket;
use std::env;
use colored::*;

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();
	let port: i32;
	let host: &str;
	if args.len() >= 2 {
		if args.contains(&String::from("--help")) {
			println!("Usage: slip [PORT] [HOST]");
			return Ok(());
		}
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
	const COLORS: [[u8; 3]; 8] = [
		[231, 42, 39], // red
		[229, 116, 38], // orange
		[203, 197, 18], // yellow
		[76, 191, 54], // green
		[16, 202, 156], // cyan or torquoise
		[18, 152, 207], // light blue
		[140, 47, 209], // ourple
		[196, 65, 150], // pink
	];
	let mut color_index: usize = 0; 
	let mut buf = [0u8; 32768];
	loop {
		let (amt, _src) = socket.recv_from(&mut buf)?;
		let color = COLORS[color_index];
		println!("{}", String::from_utf8_lossy(&buf[..amt]).truecolor(color[0], color[1], color[2]));
		color_index += 1;
		if color_index >= COLORS.len() {
			color_index = 0;
		}
	}
}
