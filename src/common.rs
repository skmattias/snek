extern crate termion;
extern crate rand;

pub mod print_tools {
    use std::io::{stdout,Write};
    use termion::raw::IntoRawMode;

    // Write functions.
    pub fn print_line(text: String) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout, "{}{}\r\n",
            text,
            termion::cursor::Hide).unwrap(); // Hide the cursor.
        stdout.flush().unwrap();
    }

    pub fn clear_and_print_line(text: String) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout, "{}{}{}{}",
            termion::clear::All, // Clear the screen.
            termion::cursor::Goto(1, 1), // Goto (1,1).
            text,
            termion::cursor::Hide).unwrap(); // Hide the cursor.
        stdout.flush().unwrap();
    }

    pub fn clear() {
        let mut stdout = stdout().into_raw_mode().unwrap();        
        write!(stdout, "{}{}", termion::cursor::Goto(1, 1), 
               termion::clear::All).unwrap();        
        stdout.flush().unwrap();
    }

    pub fn hide_cursor() {
        let mut stdout = stdout().into_raw_mode().unwrap();        
        write!(stdout, "{}", termion::cursor::Hide).unwrap();
        stdout.flush().unwrap();
    }

    pub fn show_cursor() {
        let mut stdout = stdout().into_raw_mode().unwrap();        
        write!(stdout, "{}", termion::cursor::Show).unwrap();
        stdout.flush().unwrap();
    }

    pub fn print_at_pos(c: &str, x: u16, y: u16) {
        let mut stdout = stdout().into_raw_mode().unwrap();        
        write!(stdout, "{}{}", termion::cursor::Goto(x, y), c);
        stdout.flush().unwrap();
    }
}

pub mod input_tools {
    use termion::event::Key;
    use termion::input::TermRead;
    use std::io::stdin;

    pub fn wait_for_key(key: char) {
        for c in stdin().keys() {
            match c.unwrap() {
                Key::Char(key) => break,
                _              => continue,
            }
        }
    }
}

pub mod tools {
    use common::rand::Rng;

    pub fn rand_x_y(width: u16, height: u16) -> (u16, u16) {
        // Random value in [low, high); including low but not high.
        let x: u16 = rand::thread_rng().gen_range(2, width);
        let y: u16 = rand::thread_rng().gen_range(2, height);
        (x, y)
    }
}
