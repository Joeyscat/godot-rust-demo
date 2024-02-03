use std::{
    cell::RefCell,
    io::{Read, Write},
    net::TcpStream,
    rc::Rc,
    sync::Arc,
};

use bytes::Bytes;
use godot::prelude::*;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[derive(GodotClass)]
#[class(init)]
struct Client_ {
    stream: Option<TcpStream>,
}

#[godot_api]
impl Client_ {
    #[func]
    fn connect(&mut self, addr: String, timeout_sec: u64) -> u8 {
        let timeout = std::time::Duration::from_secs(timeout_sec);
        match &self.stream {
            Some(stream) => {
                let addr = stream.peer_addr().unwrap();
                godot_print!("Client already connected to {}", &addr);
                return 2;
            }
            None => match TcpStream::connect_timeout(&addr.parse().unwrap(), timeout) {
                Ok(stream) => {
                    self.stream = Some(stream);
                    godot_print!("Client::connect({}) success", &addr);
                    0
                }
                Err(err) => {
                    godot_print!("Client::connect({}) error: {}", &addr, err);
                    1
                }
            },
        }
    }

    #[func]
    fn receive(&mut self) -> Array<u8> {
        godot_print!("Client::receive()");
        match self.stream.as_mut() {
            Some(stream) => {
                let mut buf = [0; 255];
                match stream.read(&mut buf) {
                    Ok(_) => {
                        godot_print!("Client::receive(): {:?}", buf);
                        return Array::from(&buf);
                    }
                    Err(err) => {
                        godot_print!("Client::receive() error: {}", err);
                        return Array::new();
                    }
                }
            }
            None => {
                godot_print!("Client::receive() error: not connected");
                return Array::new();
            }
        }
    }

    #[func]
    fn send(&mut self, data: Array<u8>) {
        let data = Vec::from(&data);
        godot_print!("Client::send(): {:?}", &data);
        match self.stream.as_mut() {
            Some(stream) => match stream.write(&data) {
                Ok(_) => {
                    godot_print!("Client::send() success");
                }
                Err(err) => {
                    godot_print!("Client::send() error: {}", err);
                }
            },
            None => {
                godot_print!("Client::send() error: not connected");
                return;
            }
        }
    }
}
