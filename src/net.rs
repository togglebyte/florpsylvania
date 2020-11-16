use std::net::TcpStream;
use std::thread;
use std::sync::mpsc::{self, Receiver, Sender};
use std::io::{Read, ErrorKind};

use legion::system;

use crate::message::Message;

pub type Tx = Sender<Message>;
pub type Rx = Receiver<Message>;

pub fn net_send(rx: Receiver<Message>) {
    thread::spawn(move || {
        loop {
            if let Ok(msg) = rx.recv() {
                // Send the message to the server
            }
        }
    });
}

pub fn net_receive(tx: Tx, mut sock: TcpStream) {
    thread::spawn(move || {
        let mut buffer = [0;1024];
        loop {
            match sock.read(&mut buffer) {
                Ok(0) => {} // Socket closed
                Ok(n) => {
                    // Decode message
                    // Send message to blargh
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    // Try again
                }
                Err(e) => break,
            }
        }
    });
}

#[system]
pub fn net_recv(#[resource] rx: &mut Rx) {
    if let Ok(msg) = rx.try_recv() {
        match msg {
            Message::Map(map_data) => {
            }
            Message::PlayerPos(pp) => {
            }
            Message::MapRequest(_) => {
            }
        }
    }
}
