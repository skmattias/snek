use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use common::print_tools;
use std::str;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            let received =  str::from_utf8(&data[0..size]).unwrap();
            if size > 0 {
                print_tools::print_line(format!("Received: {}", received));
            stream.write(&data[0..size]).unwrap();
            print_tools::print_line(format!("Sent: {}", received));
            }
            true
        },
            Err(_) => {
                print_tools::print_line(format!("An error occured, terminating connection with {}", stream.peer_addr().unwrap()));
                stream.shutdown(Shutdown::Both).unwrap();
                false
            }
    } {}
}

pub fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    print_tools::print_line("Server listening on port 3333".to_string());
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                print_tools::print_line(format!("New connection: {}", stream.peer_addr().unwrap()));
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                print_tools::print_line(format!("Error: {}", e));
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    
    print_tools::print_line(format!("Hellooo..."));
    // close the socket server
    drop(listener);
}
