mod graphics {
    pub const TOP_LEFT_CORNER: &'static str = "╔";
    pub const TOP_RIGHT_CORNER: &'static str = "╗";
    pub const BOTTOM_LEFT_CORNER: &'static str = "╚";
    pub const BOTTOM_RIGHT_CORNER: &'static str = "╝";
    pub const VERTICAL_WALL: &'static str = "║";
    pub const HORIZONTAL_WALL: &'static str = "═";
    pub const VERTICAL_SNAKE_BODY: &'static str = "║";
    pub const HORIZONTAL_SNAKE_BODY: &'static str = "═";
    pub const SNAKE_HEAD: &'static str = "@";
    pub const FOOD: &'static str = "o";
    pub const GAME_START_PROMPT: &'static str = "╔══════════════════════════════╗\n\r\
                                                 ║──          SNEK!           ──║\n\r\
                                                 ║──────────────────────────────║\n\r\
                                                 ║ w ┆ up                       ║\n\r\
                                                 ║ a ┆ left      Press space    ║\n\r\
                                                 ║ s ┆ down       to BEGIN!     ║\n\r\
                                                 ║ d ┆ right                    ║\n\r\
                                                 ╚═══╧══════════════════════════╝";

}

use std::io::{stdout,stdin,Write};
use termion::input::TermRead;
use termion::event::Key;

struct Game {
    width: usize,
    height: usize
}

impl Game {
    fn start(&mut self) {
        print_tools::clear_and_print_line((*graphics::GAME_START_PROMPT).to_string());
        
        let stdin = stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char(' ') => break,
                _              => continue,
            }
        }

        self.draw_board();
    }

    fn draw_board(&mut self) {
        // Draw corners.
        // Draw sides.
    }
}

struct Snek {

}

struct Food {

}


use common::print_tools;

pub fn main() {
    let mut game = Game {
        height: 40,
        width: 80,
    };
    game.start();
}
