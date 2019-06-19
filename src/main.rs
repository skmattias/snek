extern crate termion;

use std::io::{stdin,stdout,Write};
use std::time::Duration;
use std::thread;

mod client;
mod server;

use termion::*;
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;

fn main() {
    // Get the standard input stream.
    let stdin = stdin();
    // Get the standard output stream and go to raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut selection: usize = 0;
    write_options(vec!["Server", "Client", "Option3"], selection, &mut stdout);

    // Flush stdout (i.e. make the output appear).
    stdout.flush().unwrap();

    for c in stdin.keys() {
        // Clear the current line.
        write!(stdout, "{}{}", termion::cursor::Goto(1, 1), 
               termion::clear::CurrentLine).unwrap();

        // Print the key we type...
        match c.unwrap() {
            // Exit.
            Key::Char('q') => break,

            Key::Up        => if selection > 0 {
                selection -= 1;
            },
            Key::Down      => if selection < 2 {
                selection += 1;
            },
            _              => continue,
        }
        write_options(vec!["Server", "Client", "Option3"], selection, &mut stdout);

        // Flush again.
        stdout.flush().unwrap();
    }

    // Show the cursor again before we exit.
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn write_options(options: Vec<&str>, mut selected: usize, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {
    let mut to_print = String::new();
    for (i, s) in options.iter().enumerate() {
        let mut to_add = String::new();
        if i == selected {
            to_add = format!("> {}\r\n", s);
        } else {
            to_add = format!("  {}\r\n", s);
        }
        to_print.push_str(&to_add);
    }

    write!(stdout, "{}{}{}{}",
           termion::clear::All, // Clear the screen.
           termion::cursor::Goto(1, 1), // Goto (1,1).
           to_print,
           termion::cursor::Hide).unwrap(); // Hide the cursor.
}
