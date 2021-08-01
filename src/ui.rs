use ncurses::*;
use Gfx::*;

use crate::gamestate::GameState;

static IS_INITIATED: bool = false;

#[derive(Clone, Copy)]
enum Gfx {
    Head,
    Tail,
    Fruit,
    Wall,
    Space,
}

impl Gfx {
    fn draw(&self) {
        match self {
            Head  => { addstr("@@"); },
            Tail  => { addstr("<>"); },
            Fruit => { addstr("()"); },
            Wall  => { addstr("::"); },
            Space => { addstr("  "); },
        }
    }

    fn init() {
        if IS_INITIATED { return; }

        IS_INITIATED = true;
    }

    fn deinit() {
        if !IS_INITIATED { return; }

        IS_INITIATED = false;
    }

    fn render(state: &GameState) {
        let display = vec![vec![Gfx::Space; state.width as usize]; state.height as usize];

        for line in display {
            for item in line {
                item.draw();
            }
        }
    }
}

struct Ui {
    state: GameState,
}

impl Ui {
    fn new() -> Ui {
        Gfx::init();
        GameState::new(10, 10);
    }

    fn start(&mut self) {
        Gfx::init();
        Gfx::render(&self.state);
        Gfx::deinit();
    }
}
