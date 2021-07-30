fn main() {
    println!("Hello, world!");
}

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

struct Board {
    width: i32,
    height: i32,

    snake: Head,
}

impl Board {
    fn new(width: i32, height: i32) -> Board {
        Board { width, height, snake: Head { x: 0, y: 0, tail: None } }
    }
}
