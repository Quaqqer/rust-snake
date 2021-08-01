use ncurses::*;
use Gfx::*;
use std::thread::sleep;
use std::time::Duration;

use crate::gamestate::{GameState, TickResult};

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

    fn draw(&self) {
        match self {
            Head  => { addstr("@@"); },
            Tail  => { addstr("<>"); },
            Fruit => { addstr("()"); },
            Wall  => { addstr("::"); },
            Space => { addstr("  "); },
        }
    }

    fn render(state: &GameState) {
        clear();

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

        for line in display {
            for item in line {
                item.draw();
            }
        }

        refresh();
    }
}

pub struct Ui {
    window: WINDOW,
    state: GameState,
}

impl Ui {
    fn init() -> WINDOW {
        setlocale(LcCategory::ctype, "");
        let win = initscr();
        timeout(0);
        keypad(win, true);
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

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

        Ui { window: win, state: gs }
    }

    pub fn start(&mut self) {
        Ui::init();

        loop {
            Gfx::render(&self.state);
            let tr = self.state.tick();

            sleep(Duration::from_millis(100));

            if let TickResult::Exit(_score) = tr {
                break;
            }
        }

        Ui::deinit();
    }
}
