use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use common::print_tools;
use common::input_tools;
use std::str;
use std::sync::mpsc;

fn handle_client(mut stream: TcpStream) {
    loop {
        // Using a 50 byte buffer.
        let mut data = [0 as u8; 50];
        match stream.read(&mut data) {
            Ok(size) => {
                if size != 0 {
                    let received =  str::from_utf8(&data[0..size]).unwrap();
                    print_tools::print_line(format!("Received {} bytes", size));
                    stream.write(&data[0..size]).unwrap();
                    print_tools::print_line(format!("Received and echoed: {}", received));
                } else {
                    print_tools::print_line("Received 0 bytes, closing stream...".to_string());
                    break;
                }
            },
            Err(_) => {
                print_tools::print_line(format!("An error occured, terminating connection {}", 
                                                stream.peer_addr().unwrap()));
                stream.shutdown(Shutdown::Both).unwrap();
                break;
            }
        }
    }
}

pub fn main() {
    // Listen on all IP:s, port 3333.
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();

    // Accept connections and process them, spawning a new thread for each one.
    print_tools::print_line("Server listening on port 3333".to_string());

    let (n_connections_tx, n_connections_rx) = mpsc::channel();

    let connection_thread_handle = thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    // Connection succeeded.
                    print_tools::print_line(format!("New connection: {}", stream.peer_addr().unwrap()));
                    let tx = mpsc::Sender::clone(&n_connections_tx);
                    thread::spawn(move|| {
                        // When the thread is started...
                        tx.send(1).unwrap();
                        handle_client(stream);

                        // When the connection is closed...
                        tx.send(-1).unwrap();
                        print_tools::print_line(format!("Disconnected"));
                    });
                }
                Err(e) => {
                    // Connection failed.
                    print_tools::print_line(format!("Error: {}", e));
                }
            }
        }
 
        // Close the socket server.
        drop(listener);
    });

    thread::spawn(move || {
        let mut n_connections = 0;
        for received in n_connections_rx {
            n_connections += received;
            print_tools::print_line(format!("{} active connections.", n_connections));
        }
    });
    
    input_tools::wait_for_key('q');


    // Wait for the connection thread to close...
    connection_thread_handle.join().unwrap();
}
