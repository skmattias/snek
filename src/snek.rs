extern crate rand;

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
use std::thread;
use std::time::Duration;
use termion::input::TermRead;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::{async_stdin, clear, color, cursor, style};
use snek::rand::Rng;
use std::collections::VecDeque;

use self::graphics::*;

struct Game {
    width: u16,
    height: u16,
    snek: Snek,
}

impl Game {
    fn start(&mut self) {
        print_tools::clear_and_print_line((*graphics::GAME_START_PROMPT).to_string());
        input_tools::wait_for_key(' ');

        self.draw_board();
        self.snek.init(self.width, self.height);

        thread::sleep(Duration::from_secs(10));
    }

    fn draw_board(&self) {
        let width: u16 = self.width as u16;
        let height: u16 = self.height as u16;
        print_tools::clear();

        // Draw corners.
        let mut stdout = stdout().into_raw_mode().unwrap();        
        write!(stdout, "{}{}", cursor::Goto(1, 1), TOP_LEFT_CORNER).unwrap();     
        write!(stdout, "{}{}", cursor::Goto(width, 1), TOP_RIGHT_CORNER).unwrap();        
        write!(stdout, "{}{}", cursor::Goto(width, height), BOTTOM_RIGHT_CORNER).unwrap();        
        write!(stdout, "{}{}", cursor::Goto(1, height), BOTTOM_LEFT_CORNER).unwrap();

        // Draw sides.
        for x in 2..width {
            write!(stdout, "{}{}", cursor::Goto(x, 1), HORIZONTAL_WALL).unwrap();                 
            write!(stdout, "{}{}", cursor::Goto(x, height), HORIZONTAL_WALL).unwrap();                 
        }
        for y in 2..height {
            write!(stdout, "{}{}", cursor::Goto(1, y), VERTICAL_WALL).unwrap();                 
            write!(stdout, "{}{}", cursor::Goto(width, y), VERTICAL_WALL).unwrap();                 
        }

        stdout.flush().unwrap();
    }

    fn generate_food(&self) {
        // let x: u16 = rand::thread_rng().gen_range(1, self.width);
        // let y: u16 = rand::thread_rng().gen_range(1, self.height);
        // TODO check that there is no snek here...


    }
}

struct Snek {
    positions: VecDeque<(u16, u16)>,
}

impl Snek {
    fn init(&mut self, game_width: u16, game_height: u16) {
        // Randomize a starting posiiton for the snake head.
        let (x, y) = tools::rand_x_y(game_height, game_width);
        self.positions.push_front((x, y));

        // Draw snake head.
        print_tools::print_at_pos(SNAKE_HEAD, 2, 2);
    }
}

struct Food {
    x: u16,
    y: u16,
}


use common::print_tools;
use common::input_tools;
use common::tools;

pub fn main() {
    // Init a game the size of the terminal window.
    let size = termion::terminal_size().unwrap();
    let mut game = Game {
        height: size.1,
        width: size.0,
        snek: Snek {positions: VecDeque::new()},
    };
    game.start();
}
