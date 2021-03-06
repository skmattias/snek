extern crate termion;

use std::io::stdin;

mod common;
mod client;
mod server;
mod snek;

use common::print_tools;
use termion::event::Key;
use termion::input::TermRead;

fn main() {
    // Get the standard input stream.
    let stdin = stdin();
    let mut selection: usize = 0;
    let options = vec!["Server", "Client", "Single Player"];
    write_options(&options, selection);


    for c in stdin.keys() {
        print_tools::clear();

        // Print the key we type...
        match c.unwrap() {
            // Exit.
            Key::Char('q') => break,

            Key::Up        => if selection > 0 {
                selection -= 1;
            },
            Key::Down      => if selection < options.len()-1 {
                selection += 1;
            },
            Key::Char('\n')     => break,
            _              => continue,
        }
        write_options(&options, selection);
    }

    print_tools::clear();


    match selection {
        0 => server::main(),
        1 => client::main(),
        2 => snek::main(),
        _ => std::process::exit(1)
    }

    print_tools::show_cursor();
}

fn write_options(options: &Vec<&str>, selected: usize) {
    let mut to_print = String::new();
    for (i, s) in options.iter().enumerate() {
        let to_add;
        if i == selected {
            to_add = format!("> {}\r\n", s);
        } else {
            to_add = format!("  {}\r\n", s);
        }
        to_print.push_str(&to_add);
    }

    print_tools::clear_and_print_line(to_print);
}
