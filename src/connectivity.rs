use std::net::TcpStream;
use std::io::prelude::*;


#[derive(Debug)]
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum packet_types {
    CONNECTION_ATTEMPT   = 0b00000000,
    CONNECTION_ACCEPT    = 0b00000001,
    CONNECTION_REJECT    = 0b00000010,
    CONNECTION_TERMINATE = 0b00000011,
    SIMPLE_MSG           = 0b00000100,
    ART_MSG              = 0b00000101,
    ERROR_MSG            = 0b00000110,
    PING                 = 0b00000111
}

impl From<u8> for packet_types {
    fn from(value: u8) -> Self {
        match value {
            0b00000000 => packet_types::CONNECTION_ATTEMPT,
            0b00000001 => packet_types::CONNECTION_ACCEPT,
            0b00000010 => packet_types::CONNECTION_REJECT,
            0b00000011 => packet_types::CONNECTION_TERMINATE,
            0b00000100 => packet_types::SIMPLE_MSG,
            0b00000101 => packet_types::ART_MSG,
            0b00000110 => packet_types::ERROR_MSG,
            0b00000111 => packet_types::PING,
            _ => panic!("Unknown packet type!"),
        }
    }
}




// ADD ASCII ART EDITOR
/* packet are 1024 bytes is size, first 16 are header, first 1 are type, second 15 are name, last 4 are counter 1004 is msg*/
pub struct Packet{

   pub  packet_type: packet_types,
   pub  sender: Vec<u8>,
   pub  body: Vec<u8>,  
   pub  counter: Vec<u8>,
   pub  size: u128,
   pub  bytes: Vec<u8>

}

pub fn new_packet(packet_type: packet_types, sender:Vec<u8>, body: Vec<u8>, c:Vec<u8>) -> Packet{
    let mut raw_bytes: Vec<u8> = vec![packet_type as u8];
    let pad = vec![0;15-sender.len()];
    raw_bytes.extend(&sender);
    raw_bytes.extend(pad);
    raw_bytes.extend(&body);
    raw_bytes.extend(&c);
    Packet {
        packet_type : packet_type,
        sender : sender,
        body : body,
        counter : c,
        size : 1024,
        bytes: raw_bytes
    }
}

pub fn send_packet(mut stream: &TcpStream, packet: Packet) -> std::io::Result<()>{
    let res = stream.write_all(&packet.bytes);
    res
}