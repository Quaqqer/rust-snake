fn main() {
    println!("Hello, world!");
}

const GFX_BLANK: &str = "  ";
const GFX_HEAD:  &str = "@@";
const GFX_TAIL:  &str = "<>";

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn value(&self) -> (i32, i32) {
        match *self {
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
}

impl Head {
    fn dmove(&mut self, dir: Direction) {
        if let Some(tail) = &mut self.tail {
            tail.rmove(self.x, self.y);
        }

        let (dx, dy) = dir.value();
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
        Board { width, height, snake: Head { x: 0, y: 0, tail: None } }
    }
}

fn tick(board: &mut Board) {

}

fn render(board: &Board) {
    let mut screen = vec![vec![GFX_BLANK; board.width as usize]; board.height];

    board.snake.draw(&mut screen);
}
