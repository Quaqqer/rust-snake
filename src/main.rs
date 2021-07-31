use ncurses::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let w = initscr();
    timeout(0);
    keypad(w, true);

    let mut board = Board::new(20, 20);

    loop {
        let tr = tick(&mut board);
        if tr == TickResult::EXIT { break; }
        render(&board);

        sleep(Duration::from_millis(300));
    }

    endwin();
}

const GFX_BLANK: &str = "  ";
const GFX_HEAD:  &str = "@@";
const GFX_TAIL:  &str = "<>";

#[derive(PartialEq, Eq)]
enum Direction {
    NONE,
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(PartialEq, Eq)]
enum TickResult {
    CONTINUE,
    EXIT,
}

impl Direction {
    fn value(&self) -> (i32, i32) {
        match *self {
            Direction::NONE  => ( 0,  0),
            Direction::DOWN  => ( 0,  1),
            Direction::UP    => ( 0, -1),
            Direction::LEFT  => (-1,  0),
            Direction::RIGHT => ( 1,  0),
        }
    }
}

trait Growable {
    fn grow(&mut self);
}

trait Drawable {
    fn draw(&self, screen: &mut Vec<Vec<&str>>);
}

struct Head {
    x: i32,
    y: i32,

    tail: Option<Tail>,
    dir: Direction,
}

impl Head {
    fn dmove(&mut self) {
        if let Some(tail) = &mut self.tail {
            tail.rmove(self.x, self.y);
        }

        let (dx, dy) = self.dir.value();
        self.x += dx;
        self.y += dy;
    }
}

impl Growable for Head {
    fn grow(&mut self) {
        match &mut self.tail {
            Some(t) => t.grow(),
            None => self.tail = Some(Tail { x: self.x, y: self.y, tail: None }),
        }
    }
}

impl Drawable for Head {
    fn draw(&self, screen: &mut Vec<Vec<&str>>) {
        screen[self.y as usize][self.x as usize] = GFX_HEAD;

        if let Some(tail) = &self.tail {
            tail.draw(screen);
        }
    }
}

struct Tail {
    x: i32,
    y: i32,

    tail: Option<Box<Tail>>,
}

impl Tail {
    fn rmove(&mut self, x: i32, y: i32) {
        if let Some(tail) = &mut self.tail {
            tail.rmove(self.x, self.y);
        }

        self.x = x;
        self.y = y;
    }
}

impl Growable for Tail {
    fn grow(&mut self) {
        match &mut self.tail {
            Some(t) => t.grow(),
            None => self.tail = Some(Box::new(Tail { x: self.x, y: self.y, tail: None })),
        }
    }
}

impl Drawable for Tail {
    fn draw(&self, screen: &mut Vec<Vec<&str>>) {
        screen[self.y as usize][self.x as usize] = GFX_TAIL;

        if let Some(tail) = &self.tail {
            tail.draw(screen);
        }
    }
}

struct Board {
    width: usize,
    height: usize,

    snake: Head,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        Board { width, height, snake: Head { x: 0, y: 0, tail: None, dir: Direction::NONE } }
    }
}

fn tick(board: &mut Board) -> TickResult {
    loop {
        let c = getch();
        if c == -1 { break };
        println!("{}", c);

        if c == ('q' as i32) { return TickResult::EXIT };

        let dir: Direction = match c {
            KEY_UP    => Direction::UP,
            KEY_DOWN  => Direction::DOWN,
            KEY_LEFT  => Direction::LEFT,
            KEY_RIGHT => Direction::RIGHT,
            _         => Direction::NONE,
        };

        if dir != Direction::NONE {
            board.snake.dir = dir;
        }
    }

    board.snake.dmove();

    TickResult::CONTINUE
}

fn render(board: &Board) {
    let mut screen = vec![vec![GFX_BLANK; board.width as usize]; board.height];

    board.snake.draw(&mut screen);

    clear();

    for line in screen.iter_mut() {
        let l = line.join("") + "\n";
        addstr(l.as_ref());
    }

    refresh();
}
