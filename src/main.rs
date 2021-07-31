use ncurses::*;
use rand::prelude::random;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let w = initscr();
    timeout(0);
    keypad(w, true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    let (mut width, mut height): (i32, i32) = (0, 0);
    getmaxyx(w, &mut height, &mut width);

    let mut board = Board::new((width/2 - 2) as usize, (height - 2) as usize);

    loop {
        let tr = tick(&mut board);
        if tr == TickResult::EXIT { break; }
        render(&board);

        sleep(Duration::from_millis(150));
    }

    endwin();
}

const GFX_BLANK: &str = "  ";
const GFX_HEAD:  &str = "@@";
const GFX_TAIL:  &str = "<>";
const GFX_WALL:  &str = "::";
const GFX_FRUIT: &str = "()";

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

    fn contains_point(&self, x: i32, y: i32) -> bool {
        if self.x == x && self.y == y { return true; }

        if let Some(tail) = &self.tail {
            return tail.contains_point(x, y);
        }

        false
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

    fn contains_point(&self, x: i32, y: i32) -> bool {
        if self.x == x && self.y == y { return true; }

        if let Some(tail) = &self.tail {
            return tail.contains_point(x, y);
        }

        false
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
    fruit: Option<Fruit>,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        let mut b = Board { width, height, snake: Head { x: 0, y: 0, tail: None, dir: Direction::NONE }, fruit: None };
        b.spawn_fruit();

        b
    }

    fn contains_point(&self, x: i32, y: i32) -> bool {
        0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32
    }

    fn spawn_fruit(&mut self) {
        loop {
            let fx = (random::<f32>() * self.width as f32) as usize;
            let fy = (random::<f32>() * self.height as f32) as usize;

            if !self.snake.contains_point(fx as i32, fy as i32) {
                self.fruit = Some(Fruit { x: fx, y: fy });
                break;
            }
        }
    }
}

struct Fruit {
    x: usize,
    y: usize,
}

impl Drawable for Fruit {
    fn draw(&self, screen: &mut Vec<Vec<&str>>) {
        screen[self.y as usize][self.x as usize] = GFX_FRUIT;
    }
}

fn tick(board: &mut Board) -> TickResult {
    loop {
        let c = getch();
        if c == -1 { break };

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

    // If outside board
    if !board.contains_point(board.snake.x, board.snake.y) {
        return TickResult::EXIT;
    }

    // If collide with self
    let (snakex, snakey) = (board.snake.x, board.snake.y);
    if let Some(tail) = &board.snake.tail {
        if tail.contains_point(snakex, snakey) { return TickResult::EXIT; }
    }

    // If eat fruit
    if let Some(fruit) = &board.fruit {
        if board.snake.contains_point(fruit.x as i32, fruit.y as i32) {
            board.spawn_fruit();
            board.snake.grow();
        }
    }

    TickResult::CONTINUE
}

fn render(board: &Board) {
    let mut screen = vec![vec![GFX_BLANK; board.width as usize]; board.height];

    board.snake.draw(&mut screen);

    if let Some(fruit) = &board.fruit {
        fruit.draw(&mut screen);
    }

    clear();

    addstr((vec![GFX_WALL; board.width + 2].join("")).as_ref());
    for line in screen.iter_mut() {
        let l = String::from(GFX_WALL) + &line.join("") + GFX_WALL;
        addstr(l.as_ref());
    }
    addstr((vec![GFX_WALL; board.width + 2].join("")).as_ref());

    refresh();
}
