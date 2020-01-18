
extern crate enum_map;

pub type Coordinate = i8;

pub struct Map<T> {
    pub height: Coordinate,
    pub width: Coordinate,
    pub map: Vec<T>,
}

impl<T> Map<T> {
    pub fn get(&self, x: Coordinate, y: Coordinate) -> Option<&T> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }

        self.map.get((y * self.width + x) as usize)
    }
    pub fn get_mut(&mut self, x: Coordinate, y: Coordinate) -> Option<&mut T> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }

        self.map.get_mut((y * self.width + x) as usize)
    }
}

#[derive(Clone, Copy, enum_map::Enum)]
pub enum CaseState {
    Empty,
    Box,
    Wall,
}

#[derive(Clone, Copy, enum_map::Enum)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct State {
    pub map: Map<CaseState>,
    pub spots: Vec<(Coordinate, Coordinate)>,
    pub mario_x: Coordinate,
    pub mario_y: Coordinate,
    pub mario_orientation: Direction,
}

impl State {
    pub fn move_mario(&mut self, direction: Direction) {
        let (dx, dy) = match direction {
            Direction::Down => (0, 1),
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        if let Some(case) = self.map.get(self.mario_x + dx, self.mario_y + dy) {
            match case {
                CaseState::Wall => {}
                CaseState::Empty => {
                    self.mario_x += dx;
                    self.mario_y += dy;
                    self.mario_orientation = direction;
                }
                CaseState::Box => {
                    if let Some(case2) = self.map.get(self.mario_x + 2 * dx, self.mario_y + 2 * dy)
                    {
                        match case2 {
                            CaseState::Wall | CaseState::Box => {}
                            CaseState::Empty => {
                                *self
                                    .map
                                    .get_mut(self.mario_x + dx, self.mario_y + dy)
                                    .unwrap() = CaseState::Empty;
                                *self
                                    .map
                                    .get_mut(self.mario_x + 2 * dx, self.mario_y + 2 * dy)
                                    .unwrap() = CaseState::Box;
                                self.mario_x += dx;
                                self.mario_y += dy;
                                self.mario_orientation = direction;
                            }
                        }
                    }
                }
            }
        }
    }
}
