extern crate rand;
extern crate single_value_channel;

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
use termion::{cursor};
use snek::rand::Rng;
use std::collections::VecDeque;
use self::single_value_channel::channel_starting_with;

use self::graphics::*;

struct Game {
    width: u16,
    height: u16,
    snake: Snek,
    food: Vec<Food>,
    score: i32,
}

impl Game {
    fn start(&mut self) {
        self.draw_board();
        self.snake.init(self.width, self.height);

        let mut eat = false;
        loop {
            // Move the snake a step.
            self.snake.step(eat);

            
            // Check for collisions.
            if !self.check_collisions() {
                break;
            }

            // Check for eaten food the last step.
            if self.try_eat() {
                self.score += 1;
                eat = true;
            } else {
                eat = false;
            }

            // Gemerate new food, 20% chance.
            self.generate_food(0.2);

            // Wait until the next step.
            thread::sleep(Duration::from_millis(100));
        }

        print_tools::print_at_pos(&format!("SCORE: {}", self.score), self.width/2, self.height/2);
        print_tools::print_at_pos("PRESS Q TO QUIT", self.width/2, self.height/2 + 1);
        input_tools::wait_for_any_key();
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

    fn check_collisions(&self) -> bool {
        self.check_borders() && self.check_snake_collision()
    }

    fn check_borders(&self) -> bool {
        self.snake.positions.iter().all(|&(x, y, _part)| {
            x > 1 && 
            x < self.width &&
            y > 1 &&
            y < self.height 
        })
    }

    fn check_snake_collision(&self) -> bool {
        let &(h_x, h_y, _part) = self.snake.positions.front().unwrap();
        let c = self.snake.positions.iter()
            .filter(|&(x, y, _part)| *x == h_x && *y == h_y)
            .count();
        
        c <= 1
    }

    fn try_eat(&mut self) -> bool {
        let &(h_x, h_y, _part) = self.snake.positions.front().unwrap();
        if self.food.iter().any(|f| f.x == h_x && f.y == h_y) {
            self.food.retain(|&f| !(f.x == h_x && f.y == h_y));
            return true;
        }
        false
    }

    fn generate_food(&mut self, chance: f32) {
        // 20% chance to generate food.
        let result = rand::thread_rng().gen_range(0.0, 1.0);
        if result < chance {
            // Randomize the food position, try again if there is a snake there.
            let mut pos = tools::rand_x_y(self.width, self.height);
            while  self.snake.positions.iter().any(|&(sx, sy, _part)| sx == pos.0 && sy == pos.1) {
                pos = tools::rand_x_y(self.width, self.height);
            }

            let food = Food {x: pos.0, y: pos.1};
            self.food.push(food);
            print_tools::print_at_pos(FOOD, pos.0, pos.1);
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down, 
    Left,
}

struct Snek {
    // x, y, snake_part.
    positions: VecDeque<(u16, u16, &'static str)>,
    direction: Direction,
    single_rx: single_value_channel::Receiver<Direction>
}

impl Snek {
    fn init(&mut self, game_width: u16, game_height: u16) {
        // Randomize a starting posiiton for the snake head.
        let (x, y) = tools::rand_starting_position(game_width, game_height);
        self.positions.push_front((x, y, HORIZONTAL_SNAKE_BODY));

        // Create the tail..
        self.positions.push_back((x-1, y, HORIZONTAL_SNAKE_BODY));
        self.positions.push_back((x-2, y, HORIZONTAL_SNAKE_BODY));
        self.positions.push_back((x-3, y, HORIZONTAL_SNAKE_BODY));
        self.positions.push_back((x-4, y, HORIZONTAL_SNAKE_BODY));
        self.positions.push_back((x-5, y, HORIZONTAL_SNAKE_BODY));
    }

    fn step(&mut self, eat: bool) {
        
        // Get the new direction.
        let mut new_dir = *self.single_rx.latest();
        let old_dir = self.direction;

        // Update the direction.
        if (old_dir == Direction::Right && new_dir == Direction::Left) ||
           (old_dir == Direction::Left && new_dir == Direction::Right) ||
           (old_dir == Direction::Up && new_dir == Direction::Down) ||
           (old_dir == Direction::Down && new_dir == Direction::Up) {
            new_dir = old_dir;
        } else {
            self.direction = new_dir;
        }

        // Get the old head.
        let &(old_head_x, old_head_y, _old_head_snake_part) = 
            self.positions.front().unwrap();
        
        let (mut new_head_x, mut new_head_y) = (old_head_x, old_head_y);

        // Push the new head.
        match new_dir {
            Direction::Up => new_head_y -= 1,
            Direction::Down => new_head_y += 1,
            Direction::Left => new_head_x -= 1,
            Direction::Right => new_head_x += 1,
        };
        self.positions.push_front((new_head_x, new_head_y, SNAKE_HEAD));

        // Set the old head snake part.
        let neck_part = dir_to_snake_part(old_dir, new_dir);
        self.positions.remove(1);
        self.positions.insert(1, (old_head_x, old_head_y, neck_part));

        // Remove the old end of the tail.
        if !eat {
            let old_tail = self.positions.pop_back().unwrap();
            print_tools::print_at_pos(" ", old_tail.0, old_tail.1);
        }
        
        // Draw.
        for &(x, y, part) in self.positions.iter() {
            print_tools::print_at_pos(part, x, y);
        }
    }
}

fn dir_to_snake_part(old: Direction, new: Direction) -> &'static str {
    if        old == Direction::Right && new == Direction::Right ||
                old == Direction::Left && new == Direction::Left {
        return HORIZONTAL_SNAKE_BODY;
    } else if old == Direction::Up && new == Direction::Up ||
                old == Direction::Down && new == Direction::Down {
        return VERTICAL_SNAKE_BODY;
    } else if old == Direction::Right && new == Direction::Down ||
                old == Direction::Up && new == Direction::Left {
        return TOP_RIGHT_CORNER;
    } else if old == Direction::Left && new == Direction::Down ||
                old == Direction::Up && new == Direction::Right {
        return TOP_LEFT_CORNER;
    } else if old == Direction::Right && new == Direction::Up ||
                old == Direction::Down && new == Direction::Left {
        return BOTTOM_RIGHT_CORNER;
    } else if old == Direction::Left && new == Direction::Up ||
                old == Direction::Down && new == Direction::Right {
        return BOTTOM_LEFT_CORNER;
    } else {
        return " ";
    }
}

#[derive(Copy, Clone)]
struct Food {
    x: u16,
    y: u16,
}


use common::print_tools;
use common::input_tools;
use common::tools;

pub fn main() {
    let (receiver, updater) = channel_starting_with(Direction::Right);

    // Init a game the size of the terminal window.
    let size = termion::terminal_size().unwrap();
    let mut game = Game {
        height: size.1,
        width: size.0,
        snake: Snek {
            positions: VecDeque::new(), 
            direction: Direction::Right,
            single_rx: receiver,
        },
        food: Vec::new(),
        score: 0,
    };

    print_tools::clear_and_print_line((*graphics::GAME_START_PROMPT).to_string());
    input_tools::wait_for_key(' ');

    thread::spawn(move || {
        for c in stdin().keys() {
            match c.unwrap() {
                Key::Up     => updater.update(Direction::Up).unwrap(),
                Key::Down   => updater.update(Direction::Down).unwrap(),
                Key::Left   => updater.update(Direction::Left).unwrap(),
                Key::Right  => updater.update(Direction::Right).unwrap(),
                Key::Char('q') => {
                    print_tools::clear();
                    break;
                },
                _           => continue,
            }
        }
    });

    game.start();
}
