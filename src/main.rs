mod tcp_packet_defs;

use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::io::{Read, stdin, stdout, Write};
use std::str;

fn handle_tcp(mut stream: TcpStream){
    loop{
        // byte 0: payload length n
        // byte 1 - n: payload
        let mut payload_len: [u8; 1] = [0;1];
        let mut payload: [u8; 256] = [0;256];
        stream.read_exact(&mut payload_len).expect("Lost connection to client");
        let mut payload_len_byte = payload_len[0].clone() as usize;
        let mut payload_slice:&mut [u8] = &mut payload[..payload_len_byte];
        stream.read_exact(payload_slice);

        let payload_str = str::from_utf8(&payload_slice).expect("Can't parse payload to UTF8");
        println!("IN  > {}", payload_str);

        let response_string = format!("ACK: {}", payload_str);
        let response_len = response_string.len() as u8;
        let response_len_arr = [response_len];
        stream.write(&response_len_arr);
        stream.write(response_string.as_bytes());
        println!("OUT < {}", response_string);
    }
}


fn main() -> std::io::Result<()> {
    print!("Host IP:Port > ");
    stdout().flush();

    let mut server_addr = String::new();
    stdin().read_line(&mut server_addr).expect("Error reading stdin");

    let listener = TcpListener::bind(&server_addr.trim()).expect("Failed to bind TCP socket");

    println!("TCP listener started on {}", listener.local_addr().unwrap());
    for stream in listener.incoming() {
        handle_tcp(stream?);
    }
    Ok(())
}
