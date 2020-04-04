// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    pos: (i32, i32),
    dir: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, dir: Direction) -> Self {
        Robot { pos: (x, y), dir }
    }

    pub fn turn_right(self) -> Self {
        let dir = match self.dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Robot { dir, ..self }
    }

    pub fn turn_left(self) -> Self {
        let dir = match self.dir {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
        Robot { dir, ..self }
    }

    pub fn advance(self) -> Self {
        let (x, y) = self.pos;
        let pos = match self.dir {
            Direction::North => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y - 1),
            Direction::West => (x - 1, y),
        };
        Robot { pos, ..self }
    }

    fn execute(self, inst: char) -> Self {
        match inst {
            'R' => self.turn_right(),
            'L' => self.turn_left(),
            'A' => self.advance(),
            _ => panic!("unsupported instruction"),
        }
    }

    pub fn instructions(self, insts: &str) -> Self {
        insts.chars().fold(self, |robot, inst| robot.execute(inst))
    }

    pub fn position(&self) -> (i32, i32) {
        self.pos
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
