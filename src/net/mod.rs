use std::net::TcpStream;
use std::thread;
use std::sync::mpsc::{self, Receiver, Sender};
use std::io::{Read, ErrorKind};

use legion::system;

use crate::message::Message;

pub type Tx = Sender<Message>;
pub type Rx = Receiver<Message>;

pub struct FakeSocket {
    tx: Tx,
    rx: Rx
}

impl FakeSocket {
    pub fn new(rx: Rx, tx: Tx) -> Self {
        Self {
            tx,
            rx,
        }
    }
}
