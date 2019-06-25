extern crate termion;
use std::net::{TcpStream};
use std::str::from_utf8;
use std::io::{stdin,stdout,Write,Read};
use common::print_tools;
use termion::*;
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;

pub fn main() {

    let mut stream = TcpStream::connect("localhost:3333").unwrap();

    // match stream {
    //     Ok(_) => {
    //         print_tools::print_line("Successfully connected to server in port 3333".to_string());
    //     },
    //     Err(e) => {
    //         print_tools::print_line(format!("Failed to connect: {}", e));
    //     }
    // };

    loop {
        let pressed_key = read_key();
        if pressed_key == "quit" {
            print_tools::print_line(format!("Exiting!"));
            break;
        }
        send_key(&stream, &pressed_key);

        receive_player_positions(&stream)

    }

    print_tools::print_line("Terminated".to_string());
}

fn receive_player_positions(mut stream : &TcpStream) {

    let mut data = [0 as u8; 6]; // using 6 byte buffer
    match stream.read_exact(&mut data) {
        Ok(_) => {
            let received = from_utf8(&data).unwrap();
            // print_tools::print_line(format!("Received {}", received));
            // if received == pressed_key {
                print_tools::print_line("Reply is ok!".to_string());
            // } else {
                // let text = from_utf8(&data).unwrap();
                // print_tools::print_line(format!("Unexpected reply: {}", text));
            // }
        },
        Err(e) => {
            print_tools::print_line(format!("Failed to receive data: {}", e));
        }
    }

}

fn read_key() -> String {
    let stdin = stdin();
    let mut pressed_key = "";

    for c in stdin.keys() {
        let go: bool = false;

        // Print the key we type...
        match c.unwrap() {
            // Exit.
            Key::Esc     => {
                pressed_key = "quit"; 
                break;
            },
            Key::Char('q')     => {
                pressed_key = "quit"; 
                break;
            },
            Key::Up        => {
                pressed_key = "Up    ";
                break;
            },
            Key::Down        => {
                pressed_key = "Down  ";
                break;
            },
            Key::Right        => {
                pressed_key = "Right ";
                break;
            },
            Key::Left      => {
                pressed_key = "Left  ";
                break;
            },
            _              => continue,
        }
    }

    pressed_key.to_string()
}

fn send_key(mut stream : &TcpStream, msg : &String) {
    print_tools::print_line(format!("Sent '{}', waiting for reply...", msg));
    stream.write(msg.as_bytes()).unwrap();
}


// pub fn main() {
//     match TcpStream::connect("localhost:3333") {
//         Ok(mut stream) => {
//             print_tools::print_line("Successfully connected to server in port 3333".to_string());
//             let msg = b"Hello!";

//             stream.write(msg).unwrap();
//             print_tools::print_line("Sent hello, waiting for reply...".to_string());

//             let mut data = [0 as u8; 6]; // using 6 byte buffer
//             match stream.read_exact(&mut data) {
//                 Ok(_) => {
//                     if &data == msg {
//                         print_tools::print_line("Reply is ok!".to_string());
//                     } else {
//                         let text = from_utf8(&data).unwrap();
//                         print_tools::print_line(format!("Unexpected reply: {}", text));
//                     }
//                 },
//                 Err(e) => {
//                     print_tools::print_line(format!("Failed to receive data: {}", e));
//                 }
//             }
//         },
//         Err(e) => {
//             print_tools::print_line(format!("Failed to connect: {}", e));
//         }
//     }
//     print_tools::print_line("Terminated".to_string());
// }
