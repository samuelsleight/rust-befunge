#[derive(Clone, Copy, Debug)]
pub enum Delta {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug)]
pub struct Ip {
    x: u32,
    y: u32,
    w: u32,
    h: u32
}

impl Delta {
    fn values(&self) -> (i32, i32) {
        match self {
            &Delta::Left => (-1, 0),
            &Delta::Right => (1, 0),
            &Delta::Up => (0, -1),
            &Delta::Down => (0, 1),
        }
    }
}

fn clamped_add(value: u32, delta: i32, max: u32) -> u32 {
    let mut value = value as i64 + delta as i64;

    if value < 0 {
        value += max as i64
    } else if value >= max as i64 {
        value -= max as i64
    }

    value as u32
}

impl Ip {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Ip {
        Ip {
            x,
            y,
            w,
            h
        }
    }

    pub fn advance(&mut self, delta: Delta) {
        let (dx, dy) = delta.values();
        self.x = clamped_add(self.x, dx, self.w);
        self.y = clamped_add(self.y, dy, self.h);
    }

    pub fn row(&self) -> u32 {
        self.x
    }

    pub fn col(&self) -> u32 {
        self.y
    }
}
