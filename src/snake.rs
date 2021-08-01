use crate::vec2::{Vec2, Point2D};

#[derive(PartialEq, Eq)]
#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Direction {
    fn vec(&self) -> Vec2 {
        use Direction::*;

        match self {
            Up    => Vec2 { x:  0, y: -1 },
            Down  => Vec2 { x:  0, y:  1 },
            Left  => Vec2 { x: -1, y:  0 },
            Right => Vec2 { x:  1, y:  0 },
            None  => Vec2 { x:  0, y:  0 },
        }
    }
}

pub struct Snake {
    pub pos: Point2D,
    pub tail: std::collections::LinkedList<Point2D>,

    pub queued_grow: u32,
    pub dir: Direction,
}

impl Snake {
    pub fn new(width: u32, height: u32, length: usize) -> Snake {
        let head_pos = Vec2::new((width / 2) as i32, (height / 2) as i32);
        let tail = (1..length).map(|i: usize| -> Vec2 { head_pos - Vec2::new(i as i32, 0) }).collect();
        Snake { pos: head_pos, tail, dir: Direction::None, queued_grow: 0 }
    }

    pub fn tick(&mut self) {
        if self.dir == Direction::None { return; }

        if self.queued_grow == 0 {
            self.tail.pop_back();
        } else {
            self.queued_grow -= 1;
        }
        self.tail.push_front(self.pos);

        self.pos += self.dir.vec();
    }

    pub fn contains_point(&self, point: Point2D) -> bool {
        if self.pos == point { return true; }

        for segment_pos in self.tail.iter() {
            if *segment_pos == point { return true; }
        }

        false
    }
}
