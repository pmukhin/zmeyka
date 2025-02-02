use std::collections::LinkedList;

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
    body: LinkedList<Pt>,
    direction: Direction,
    width_cells: i32,
    height_cells: i32,
}

impl Snake {
    pub fn new(width_cells: i32, height_cells: i32) -> Snake {
        let mut body = LinkedList::new();

        body.push_back(Pt(0, 0));
        body.push_back(Pt(1, 0));
        body.push_back(Pt(2, 0));

        Snake {
            body,
            direction: Default::default(),
            width_cells,
            height_cells,
        }
    }

    pub fn reset(&mut self) {
        self.body = LinkedList::new();
        self.body.push_back(Pt(0, 0));
        self.body.push_back(Pt(1, 0));
        self.body.push_back(Pt(2, 0));

        self.direction = Default::default();
    }

    pub fn head(&self) -> &Pt {
        self.body.front().unwrap()
    }

    pub fn len(&self) -> usize {
        self.body.len()
    }

    pub fn collapsed_into_self(&self) -> bool {
        for elem in self.body.iter().skip(1) {
            if elem == self.head() {
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
                    new_y = self.height_cells - 1;
                } else {
                    new_y -= 1
                }
            }
            Direction::Down => {
                if new_y == self.height_cells - 1 {
                    new_y = 0;
                } else {
                    new_y += 1
                }
            }
            Direction::Left => {
                if new_x == 0 {
                    new_x = self.width_cells - 1;
                } else {
                    new_x -= 1
                }
            }
            Direction::Right => {
                if new_x == self.width_cells - 1 {
                    new_x = 0;
                } else {
                    new_x += 1
                }
            }
        }

        self.body.push_front(Pt(new_x, new_y)); // O(1)
        if !growing {
            // also O(1) as this LL impl keeps a pointer to the
            // penultimate element
            self.body.pop_back();
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
