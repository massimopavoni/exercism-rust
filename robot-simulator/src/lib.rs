#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    pub p: (i32, i32),
    pub d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { p: (x, y), d }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        self.d = match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        self.d = match self.d {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        match self.d {
            Direction::North => self.p.1 += 1,
            Direction::East => self.p.0 += 1,
            Direction::South => self.p.1 -= 1,
            Direction::West => self.p.0 -= 1,
        };
        self
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .bytes()
            .fold(self, |robot, instruction| match instruction {
                b'L' => robot.turn_left(),
                b'R' => robot.turn_right(),
                b'A' => robot.advance(),
                _ => unreachable!("Invalid instruction"),
            })
    }

    pub fn position(&self) -> (i32, i32) {
        self.p
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
