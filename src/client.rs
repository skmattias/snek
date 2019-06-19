use std::net::{TcpStream};
use std::str::from_utf8;
use std::io::{stdin,stdout,Write,Read};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use common::print_tools;

pub fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    

    match TcpStream::connect("localhost:3333") {

        Ok(mut stream) => {
            
            print_tools::print_line("Successfully connected to server in port 3333".to_string());
            let msg = b"Hello!";

            stream.write(msg).unwrap();
            print_tools::print_line("Sent hello, waiting for reply...".to_string());

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        print_tools::print_line("Reply is ok!".to_string());
                    } else {
                        let text = from_utf8(&data).unwrap();
                        print_tools::print_line(format!("Unexpected reply: {}", text));
                    }
                },
                Err(e) => {
                    print_tools::print_line(format!("Failed to receive data: {}", e));
                }
            }
        },
        Err(e) => {
            print_tools::print_line(format!("Failed to connect: {}", e));
        }
    }
    print_tools::print_line("Terminated".to_string());
}
