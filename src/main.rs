use std::f32::consts::PI;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::os::raw;
use std::usize;
use std::vec;
use console::Term;
use console::style;


#[derive(Debug)]
#[derive(Copy, Clone)]
#[repr(u8)]
enum packet_types {
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
struct Packet{

    packet_type: packet_types,
    sender: Vec<u8>,
    body: Vec<u8>,  
    counter: Vec<u8>,
    size: u128,
    bytes: Vec<u8>

}

fn new_packet(packet_type: packet_types, sender:Vec<u8>, body: Vec<u8>, c:Vec<u8>) -> Packet{
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

fn send_packet(mut stream: TcpStream, packet: Packet) -> std::io::Result<()>{
    let res = stream.write_all(&packet.bytes);
    res
}

fn main() -> std::io::Result<()>{
    let term = Term::stdout();
    term.write_line("Choose an option:");
    term.write_line("   [1] Listen");
    term.write_line("   [2] Connect");

    let choice = term.read_char().unwrap();
    
    match choice {
        '1' => listen()?,
        '2' => {
            term.write_line("Choose an option:");
            term.write_line("   [1] CONNECTION_ATTEMPT");
            term.write_line("   [2] CONNECTION_ACCEPT");
            term.write_line("   [3] CONNECTION_REJECT");
            term.write_line("   [4] CONNECTION_TERMINATE");
            term.write_line("   [5] SIMPLE_MSG");
            term.write_line("   [6] ART_MSG");
            term.write_line("   [7] ERROR_MSG");
            term.write_line("   [8] PING");

            let choice = term.read_char().unwrap();
            
            match choice {
            '1' => test_connect(packet_types::CONNECTION_ATTEMPT)?,
            '2' => test_connect(packet_types::CONNECTION_ACCEPT)?,
            '3' => test_connect(packet_types::CONNECTION_REJECT)?,
            '4' => test_connect(packet_types::CONNECTION_TERMINATE)?,
            '5' => test_connect(packet_types::SIMPLE_MSG)?,
            '6' => test_connect(packet_types::ART_MSG)?,
            '7' => test_connect(packet_types::ERROR_MSG)?,
            '8' => test_connect(packet_types::PING)?,
                _=> println!("faah"),
            }

        },
        _ => println!("{}: please choose a valid alternative.", style("ERROR").red()),
    }

    Ok(())
}


fn test_connect(t: packet_types) -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:23232")?;
    println!("Sending over {}.", style("data").red());
    
    let sender = "Samuel";
    let body = vec![0;1004];
    let c=  vec![1;4];

    let p1 = new_packet(t, sender.as_bytes().to_vec(), body, c);

    stream.write(&p1.bytes)?;

    Ok(())
    }


fn listen() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:23232").unwrap();
    println!("Listening on {}.", style("127.0.0.1:23232").red());
    
    for conn in listener.incoming(){
        match conn  {
            Ok(mut conn) => {
                handle_stream(conn)?;
            },
            Err(e) => {println!("Connection failed. {}", e)},
        }
    }
    Ok(())
}

fn handle_stream(mut stream: TcpStream) -> std::io::Result<()>{
    let mut buf = vec![0;1024];
    stream.read_exact(&mut buf)?;

    let packet_type = buf[0];
    let sender =  &buf[1..16];
    let body = &buf[16..1020];
    let count = &buf[1020..1024]; 

    match packet_types::from(packet_type) {
        packet_types::CONNECTION_ATTEMPT  => {
            println!("CONNECTION_ATTEMPT recieved, attempting to establish link ...")
            handshake(1, sender, body, stream)?;
        },
        packet_types::CONNECTION_ACCEPT   => println!("ERROR: ACCEPT PACKET RECIEVED WITHOUT INITIALIZED HANDSHAKE"),
        packet_types::CONNECTION_REJECT   => println!("ERROR: REJECT PACKET RECIEVED WITHOUT INITIALIZED HANDSHAKE"),
        packet_types::CONNECTION_TERMINATE=> println!("CONNTERM"),
        packet_types::SIMPLE_MSG          => println!("SIMPLMSG"),
        packet_types::ART_MSG             => println!("ARTMSG"),
        packet_types::ERROR_MSG           => println!("ERRORMSG"),
        packet_types::PING                => println!("PING"),
    }

    Ok(())
}


fn handshake(party: u8, sender: &[u8], body: &[u8], stream: TcpStream) -> std::io::Result<()> {
    match party {
        0 => {

        }, //caller is client

        1 => {
            send_packet(stream, Packet { packet_type: (packet_types::CONNECTION_ACCEPT), sender: (), body: (), counter: (), size: (), bytes: () }); // send CONNECTION ACCEPT PACKET

        }, //caller is server

        _ => println!("{}", style("ERROR IN HANDSHAKE").red()),
    }

    Ok(())
}