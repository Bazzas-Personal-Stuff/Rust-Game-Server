mod tcp_packet_defs;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, stdin, stdout, Write};
use std::str;
use clap::{Arg, App};

fn handle_tcp(mut stream: TcpStream){
    loop{
        // byte 0: payload length n
        // byte 1 - n: payload


        // TODO: change payload_len to payload definitions enum to determine expected length/parsing behaviour
        // create input buffers
        let mut payload_len: [u8; 1] = [0;1];
        let mut payload: [u8; 256] = [0;256];
        // receive packet payload length
        stream.read_exact(&mut payload_len).expect("disconnect");
        let payload_len_byte = payload_len[0].clone() as usize;
        // slice payload buffer to the required size
        let payload_slice:&mut [u8] = &mut payload[..payload_len_byte];
        stream.read_exact(payload_slice).expect("Received payload does not adhere to game protocol");
        // parse payload to readable string
        let payload_str = str::from_utf8(&payload_slice).expect("Can't parse payload to UTF8");
        println!("IN  > {}", payload_str);


        let response_string = format!("ACK: {}", payload_str);
        let response_len = response_string.len() as u8;
        let response_len_arr = [response_len];

        // TODO: Compose packet before writing twice
        // [ u16 packet_type ] [           payload           ]

        stream.write(&response_len_arr).expect("TCP Definition Write Error");
        stream.write(response_string.as_bytes()).expect("TCP Payload Write error");
        println!("OUT < {}", response_string);
    }
}


fn main() -> std::io::Result<()> {
    // command line args
    let matches = App::new("Rust Game Server")
        .author("Bailey Gibbons <bazzagibbs.github.io>")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .takes_value(true))
        .get_matches();


    let mut server_addr ;
    let mut port_buf = String::new();
    print!("Port > ");
    stdout().flush().expect("stdout flush error");
    if matches.is_present("port"){
        server_addr = format!("localhost:{}", matches.value_of("port").unwrap());
    } else {
        stdin().read_line(&mut port_buf).expect("Error reading stdin");
        server_addr = format!("localhost:{}", port_buf);
    }


    let listener = TcpListener::bind(&server_addr.trim()).expect("Failed to bind TCP socket");

    println!("TCP listener started on {}", listener.local_addr().unwrap());
    loop {
        for stream in listener.incoming() {
            handle_tcp(stream?);
        }
    }
    // Ok(())
}
