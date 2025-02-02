use crate::CELL_SIZE;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::Down
    }
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub struct Pt(pub i32, pub i32);

impl Default for Pt {
    fn default() -> Pt {
        Pt(1, 1)
    }
}

pub struct Snake {
    body: Vec<Pt>,
    direction: Direction,
    width_cells: i32,
    height_cells: i32,
}

impl Snake {
    pub fn new(width_cells: i32, height_cells: i32) -> Snake {
        Snake {
            body: vec![Pt(0, 0), Pt(1, 0), Pt(2, 0)],
            direction: Default::default(),
            width_cells,
            height_cells,
        }
    }

    pub fn reset(&mut self) {
        self.body = vec![Pt(0, 0), Pt(1, 0), Pt(2, 0)];
        self.direction = Default::default();
    }

    pub fn head(&self) -> Pt {
        self.body[0]
    }

    pub fn len(&self) -> usize {
        self.body.len()
    }

    pub fn collapsed_into_self(&self) -> bool {
        for i in 1..self.body.len() {
            if self.body[i] == self.body[0] {
                return true;
            }
        }
        false
    }

    pub fn make_move(&mut self, growing: bool) {
        let head = self.head();
        let mut new_x = head.0;
        let mut new_y = head.1;

        // Move the head based on the direction
        match self.direction {
            Direction::Up => {
                if new_y == 0 {
                    new_y = self.height_cells-1;
                } else {
                    new_y -= 1
                }
            }
            Direction::Down => {
                if new_y == self.height_cells-1{
                    new_y = 0;
                } else {
                    new_y += 1
                }
            }
            Direction::Left => {
                if new_x == 0 {
                    new_x = self.width_cells-1;
                } else { new_x -= 1 }
            }
            Direction::Right => {
                if new_x == self.width_cells-1 {
                    new_x = 0;
                } else {
                    new_x += 1
                }
            }
        }

        // Move the body: shift all segments forward
        self.body.insert(0, Pt(new_x, new_y)); // New head
        if !growing {
            self.body.pop(); // Remove the last segment (unless growing)
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if self.direction == Direction::Right && direction == Direction::Left {
            return;
        }
        if self.direction == Direction::Left && direction == Direction::Right {
            return;
        }
        if self.direction == Direction::Down && direction == Direction::Up {
            return;
        }
        if self.direction == Direction::Up && direction == Direction::Down {
            return;
        }

        self.direction = direction;
    }

    pub fn draw<F>(&self, mut draw_fn: F)
    where
        F: FnMut(i32, i32),
    {
        for pt in &self.body {
            draw_fn(pt.0, pt.1);
        }
    }
}
