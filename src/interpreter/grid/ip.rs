#[derive(Clone, Copy, Debug)]
pub enum Delta {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug)]
pub struct Ip {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Delta {
    fn values(self) -> (i32, i32) {
        match self {
            Delta::Left => (-1, 0),
            Delta::Right => (1, 0),
            Delta::Up => (0, -1),
            Delta::Down => (0, 1),
        }
    }
}

fn clamped_add(value: usize, delta: i32, max: usize) -> usize {
    if delta < 0 {
        let delta = delta.abs() as usize;

        if delta > value {
            (value + max) - delta
        } else {
            value - delta
        }
    } else {
        let result = value + delta as usize;

        if result >= max {
            result - max
        } else {
            result
        }
    }
}

impl Ip {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self { x, y, w, h }
    }

    pub fn advance(&mut self, delta: Delta) {
        let (dx, dy) = delta.values();
        self.x = clamped_add(self.x, dx, self.w);
        self.y = clamped_add(self.y, dy, self.h);
    }

    pub fn row(&self) -> usize {
        self.x
    }

    pub fn col(&self) -> usize {
        self.y
    }
}
