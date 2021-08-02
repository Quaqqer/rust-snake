use ncurses::*;
use Gfx::*;
use std::thread::sleep;
use std::time::Duration;

use crate::{gamestate::{GameState, TickResult}, snake::Direction};

#[derive(Clone, Copy)]
enum Gfx {
    Head,
    Tail,
    Fruit,
    Wall,
    Space,
}

impl Gfx {
    const SPRITE_WIDTH: u32 = 2;

    fn init() {
        start_color();
        init_pair(1, COLOR_WHITE, COLOR_BLACK);
        init_pair(2, COLOR_BLACK, COLOR_GREEN);
        init_pair(3, COLOR_YELLOW, COLOR_GREEN);
        init_pair(4, COLOR_WHITE, COLOR_RED);
        init_pair(5, COLOR_WHITE, COLOR_BLACK | 0b1000);
        attron(COLOR_PAIR(1));
    }

    fn draw(&self) {
        match self {
            Head  => { attron(COLOR_PAIR(2)); addstr("@@"); attroff(COLOR_PAIR(2));},
            Tail  => { attron(COLOR_PAIR(3)); addstr("ZZ"); attroff(COLOR_PAIR(3)); },
            Fruit => { attron(COLOR_PAIR(4)); addstr("()"); attroff(COLOR_PAIR(4)); },
            Wall  => { attron(COLOR_PAIR(5)); addstr("::"); attroff(COLOR_PAIR(5)); },
            Space => { addstr("  "); },
        }
    }

    fn render(state: &GameState, line_rest: u32) {
        erase();

        let mut display = vec![vec![Gfx::Space; state.width as usize]; state.height as usize];

        // Draw snake
        let head = state.snake.pos;
        display[head.y as usize][head.x as usize] = Gfx::Head;
        for seg in state.snake.tail.iter() {
            display[seg.y as usize][seg.x as usize] = Gfx::Tail;
        }

        // Draw fruit
        if let Some(fruit) = state.fruit {
            display[fruit.y as usize][fruit.x as usize] = Gfx::Fruit;
        }

        for _ in 0..state.width + 2 { Gfx::Wall.draw() }
        if line_rest > 0 { addstr("\n"); }
        for line in display {
            Gfx::Wall.draw();
            for item in line {
                item.draw();
            }
            Gfx::Wall.draw();
            if line_rest > 0 { addstr("\n"); }
        }
        for _ in 0..state.width + 2 { Gfx::Wall.draw() }

        refresh();
    }
}

pub struct Ui {
    window: WINDOW,
    line_rest: u32,
    state: GameState,
}

impl Ui {
    fn init() -> WINDOW {
        setlocale(LcCategory::ctype, "");
        let win = initscr();
        timeout(0);
        keypad(win, true);
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        Gfx::init();

        win
    }

    fn deinit() {
        endwin();
    }

    pub fn new() -> Ui {
        let win = Ui::init();

        let (mut width, mut height): (i32, i32) = (0, 0);
        getmaxyx(win, &mut height, &mut width);

        let gs = GameState::new(width as u32 / Gfx::SPRITE_WIDTH - 2, height as u32 - 2);

        let line_rest = width as u32 % Gfx::SPRITE_WIDTH;

        Ui { window: win, state: gs, line_rest }
    }

    pub fn start(&mut self) {
        let exit_score: i32;

        loop {
            Gfx::render(&self.state, self.line_rest);
            let tr = self.state.tick();

            sleep(Duration::from_millis(100));

            let mut dir: Option<Direction> = None;
            loop {
                let c = getch();
                if c == -1 { break; }
                if c == ('q' as i32) { return; }

                let d: Option<Direction> = match c {
                    KEY_UP    => Some(Direction::Up),
                    KEY_DOWN  => Some(Direction::Down),
                    KEY_LEFT  => Some(Direction::Left),
                    KEY_RIGHT => Some(Direction::Right),
                    _ => None,
                };

                if d.is_some() && d.unwrap().opposite() != self.state.snake.dir {
                    dir = d;
                }
            }
            if let Some(dir) = dir { self.state.snake.dir = dir }

            if let TickResult::Exit(score) = tr {
                exit_score = score;
                break;
            }
        }

        Ui::deinit();

        println!("Game over! Score: {}", exit_score);
    }
}
