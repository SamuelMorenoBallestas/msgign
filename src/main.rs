use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::os::raw;
use std::usize;
use std::vec;
use console::Term;
use console::style;
use std::thread;

use crate::connectivity::new_packet;
use crate::connectivity::send_packet;
mod connectivity;

fn main() -> std::io::Result<()>{
    let term = Term::stdout();
    term.write_line("Choose an option:");
    term.write_line("   [1] Listen");
    term.write_line("   [2] Connect");

    let choice = term.read_char().unwrap();
    
    match choice {
        '1' => listen()?,
        '2' => test_connect()?,
        _ => println!("{}: please choose a valid alternative.", style("ERROR").red()),
    }

    Ok(())
}


fn test_connect() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("192.168.68.114:23232")?;
    println!("Sending over {}.", style("data").red());
    
    let sender = "Samuel";

    handshake(0, sender, stream)?;


    Ok(())
    }


fn listen() -> std::io::Result<()> {
    let listener = TcpListener::bind("192.168.68.114:23232").unwrap();
    println!("Listening on {}.", style("192.168.68.114:23232").red());

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

    match connectivity::packet_types::from(packet_type) {
        connectivity::packet_types::CONNECTION_ATTEMPT  => {
            println!("CONNECTION_ATTEMPT recieved, attempting to establish link ...");
            let sender = "Keona";
            handshake(1, sender, stream)?;
        },
        connectivity::packet_types::CONNECTION_ACCEPT   => println!("ERROR: ACCEPT PACKET RECIEVED WITHOUT INITIALIZED HANDSHAKE"),
        connectivity::packet_types::CONNECTION_REJECT   => println!("ERROR: REJECT PACKET RECIEVED WITHOUT INITIALIZED HANDSHAKE"),
        connectivity::packet_types::CONNECTION_TERMINATE=> println!("CONNTERM"),
        connectivity::packet_types::SIMPLE_MSG          => println!("SIMPLMSG"),
        connectivity::packet_types::ART_MSG             => println!("ARTMSG"),
        connectivity::packet_types::ERROR_MSG           => println!("ERRORMSG"),
        connectivity::packet_types::PING                => println!("PING"),
    }

    Ok(())
}


fn handshake(party: u8, sendfrom: &str, mut stream: TcpStream) -> std::io::Result<()> {
    match party {
        0 => {
            let body = vec![0;1004];
            let c=  vec![1;4];
            connectivity::send_packet(&stream, new_packet(connectivity::packet_types::CONNECTION_ATTEMPT, sendfrom.as_bytes().to_vec(), body, c))?;

            let mut answer = vec![0;1024];
            stream.read_exact(&mut answer)?;

            let packet_type = answer[0];
            let sender =  &answer[1..16];
            let body = &answer[16..1020];
            let count = &answer[1020..1024]; 
            
            match connectivity::packet_types::from(packet_type){
                connectivity::packet_types::CONNECTION_ACCEPT => {msg_active(stream); println!("Connection {}!", style("ACCEPTED").green())},
                connectivity::packet_types::CONNECTION_REJECT => {println!("Connection {}!", style("REJECTED").red()); return Ok(()); },
                _=> {println!("FUUUUUUCK")}
            }            

        }, //caller is client

        1 => {
            let body = vec![0;1004];
            let c=  vec![1;4];
            connectivity::send_packet(&stream, new_packet(connectivity::packet_types::CONNECTION_ACCEPT, sendfrom.as_bytes().to_vec(), body, c))?;
            println!("Connection {}!", style("ESTABLISHED").green());
            msg_active(stream);
        }, //caller is server

        _ => println!("{}", style("ERROR IN HANDSHAKE").red()),
    }

    Ok(())
}

fn msg_active(stream: TcpStream){
    
    let writestream = stream.try_clone().unwrap();
    let mut readstream = stream;

    thread::spawn(move ||{
        loop{
        let c=  vec![1;4];
        let term = Term::stdout();
        let choice = term.read_line().unwrap();
        let mut body = choice.as_bytes().to_vec();
        let pad = vec![0;1004-choice.len()];
        body.extend(pad);
        let p =  new_packet(connectivity::packet_types::SIMPLE_MSG, "Samuel".as_bytes().to_vec(), body, c);
        send_packet(&writestream, p);} 
    });

    loop{
        let mut answer = vec![0;1024];
        readstream.read_exact(&mut answer);
    
        let packet_type = answer[0];
        let sender =  &answer[1..16];
        let body = &answer[16..1020];
        let count = &answer[1020..1024]; 
    
        let term = Term::stdout();
        term.write_line(str::from_utf8(body).unwrap());}

}


