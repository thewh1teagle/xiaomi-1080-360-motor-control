mod ptz;

#[macro_use]
extern crate dlopen_derive;
extern crate websocket;

use ptz::PtzApi;
use std::net::TcpStream;
use websocket::sync::{Server, Writer};
use websocket::{Message, OwnedMessage, message};
use serde_json;
use std::thread;

fn handle_message(ptz: &PtzApi, mut sender: Writer<TcpStream>, message: &str) -> serde_json::Result<()> {
	let v: serde_json::Value = serde_json::from_str(message)?;
	let command = v.get("command").unwrap().as_str().unwrap();
	match command {
		"ptz_absolute" => {
			let pan = v.get("pan").unwrap().as_i64().unwrap();
			let tilt = v.get("tilt").unwrap().as_i64().unwrap();
			println!("pan: {:?} tilt: {:?}", pan, tilt);
            ptz.pan_abs(pan as f32);
            ptz.tilt_abs(tilt as f32);
		}
		"ptz_relative" => {
			let pan = v.get("pan").unwrap().as_i64();
			let tilt = v.get("tilt").unwrap().as_i64();
		}
		"ptz_info" => {
			sender.send_message(&Message::text("worked"));
		}
		_ => println!("unknown..")
	}
	Ok(())
}

fn main() {
	let server = Server::bind("0.0.0.0:2794").unwrap();
    let mut ptz = 
    PtzApi::new(0.0,360.0,166.0, -20.0, 90.0, 40.0);
    ptz.calibrate();

    
	for request in server.filter_map(Result::ok) {
        thread::spawn(move || {
            let mut client = request.accept().unwrap();

            let ip = client.peer_addr().unwrap();
    
            println!("Connection from {}", ip);
    
            // let message = OwnedMessage::Text("Hello".to_string());
            // client.send_message(&message).unwrap();
            
            let (mut receiver, mut sender) = client.split().unwrap();
    
            for message in receiver.incoming_messages() {
                let message = message.unwrap();
                match message {
                    OwnedMessage::Text(txt) => {
                        handle_message(&ptz, sender, &txt);
                    }
                    OwnedMessage::Close(_) => {
                        let message = OwnedMessage::Close(None);
                        sender.send_message(&message).unwrap();
                        println!("Client {} disconnected", ip);
                        return;
                    }
                    OwnedMessage::Ping(ping) => {
                        let message = OwnedMessage::Pong(ping);
                        sender.send_message(&message).unwrap();
                    }
                    _ => sender.send_message(&message).unwrap(),
                }
            }
        });

	}
}
