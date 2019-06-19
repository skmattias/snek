extern crate termion;

pub mod print_tools {
    use std::io::{stdout,stdin,Write};
    use termion::raw::IntoRawMode;
    use termion::input::TermRead;
    use termion::{async_stdin, clear, color, cursor, style};

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
}

