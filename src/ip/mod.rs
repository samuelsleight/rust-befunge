#[deriving(Show, Clone, Copy, PartialEq, Eq)]
pub struct IP {
    pub x: int,
    pub y: int,
    pub dx: int,
    pub dy: int
}

impl IP {
    pub fn new(x: int, y: int, dx: int, dy: int) -> IP {
        IP {
            x: x,
            y: y,
            dx: dx,
            dy: dy
        }
    }

    pub fn new_straight(&self, width: uint, height: uint) -> IP {
        let mut new = self.clone();
        new.advance(width, height);
        new
    }

    pub fn new_turn_right(&self, width: uint, height: uint) -> IP {
        let mut new = self.clone();
        new.rotate_right();
        new.advance(width, height);
        new
    }

    pub fn new_turn_left(&self, width: uint, height: uint) -> IP {
        let mut new = self.clone();
        new.rotate_left();
        new.advance(width, height);
        new
    }

    pub fn new_left(&self, width: uint, height: uint) -> IP {
        let mut new = self.clone();
        new.left();
        new.advance(width, height);
        new
    }

    pub fn new_right(&self, width: uint, height: uint) -> IP {
        let mut new = self.clone();
        new.right();
        new.advance(width, height);
        new
    }

    pub fn new_up(&self, width: uint, height: uint) -> IP {
        let mut new = self.clone();
        new.up();
        new.advance(width, height);
        new
    }

    pub fn new_down(&self, width: uint, height: uint) -> IP {
        let mut new = self.clone();
        new.down();
        new.advance(width, height);
        new
    }

    pub fn delta(&self) -> (int, int) {
        (self.dx, self.dy)
    }

    pub fn advance(&mut self, width: uint, height: uint) {
        self.x += self.dx as int;
        self.y += self.dy as int;

        if self.x >= width as int {
            self.x -= width as int
        } else if self.x < 0 {
            self.x += width as int
        }

        if self.y >= height as int {
            self.y -= height as int
        } else if self.y < 0 {
            self.y += height as int
        }
    }

    pub fn left(&mut self) {
        self.dx = -1;
        self.dy = 0;
    }

    pub fn right(&mut self) {
        self.dx = 1;
        self.dy = 0;
    }

    pub fn up(&mut self) {
        self.dx = 0;
        self.dy = -1;
    }

    pub fn down(&mut self) {
        self.dx = 0;
        self.dy = 1;
    }

    pub fn rotate_right(&mut self) {
        let new_dx = self.dy;
        let new_dy = -self.dx;

        self.dx = new_dx;
        self.dy = new_dy;
    }

    pub fn rotate_left(&mut self) {
        let new_dx = -self.dy;
        let new_dy = self.dx;

        self.dx = new_dx;
        self.dy = new_dy;
    }

    pub fn flip(&mut self) {
        self.dx *= -1;
        self.dy *= -1;
    }
}

