fn main() {
    println!("Hello, world!");
}

impl Direction {
    #[must_use]
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    #[must_use]
    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Location {
    x: u8,
    y: u8,
}

#[derive(Debug, PartialEq)]
struct Robot {
    location: Location,
    direction: Direction,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Left,
    Right,
    Forward,
}

impl Robot {
    fn move_forward(&mut self) {
        self.location = match self.direction {
            Direction::North => Location {
                x: self.location.x,
                y: self.location.y + 1,
            },
            Direction::East => Location {
                x: self.location.x + 1,
                y: self.location.y,
            },
            Direction::South => Location {
                x: self.location.x,
                y: self.location.y.saturating_sub(1),
            },
            Direction::West => Location {
                x: self.location.x.saturating_sub(1),
                y: self.location.y,
            },
        }
    }

    fn turn_left(&mut self) {
        self.direction = self.direction.turn_left()
    }

    fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }

    fn follow(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            match instruction {
                Instruction::Left => self.turn_left(),
                Instruction::Right => self.turn_right(),
                Instruction::Forward => self.move_forward(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_turns_left() {
        let dir = Direction::North;
        let west = dir.turn_left();
        let south = west.turn_left();

        let expected = Direction::South;

        assert_eq!(south, expected);
    }

    #[test]
    fn direction_turns_right() {
        let dir = Direction::North;
        let east = dir.turn_right();
        let south = east.turn_right();

        let expected = Direction::South;

        assert_eq!(south, expected);
    }

    #[test]
    fn direction_moves_forward() {
        let mut robot = Robot {
            location: Location { x: 0, y: 0 },
            direction: Direction::North,
        };
        robot.move_forward();

        let expected = Location { x: 0, y: 1 };

        assert_eq!(robot.location, expected);
    }

    #[test]
    fn direction_moves_forward_south() {
        let mut robot = Robot {
            location: Location { x: 0, y: 0 },
            direction: Direction::South,
        };
        robot.move_forward();

        let expected = Location { x: 0, y: 0 };

        assert_eq!(robot.location, expected);
    }

    #[test]
    fn robot_moves_left() {
        let mut robot = Robot {
            location: Location { x: 0, y: 0 },
            direction: Direction::South,
        };
        robot.turn_left();

        let expected = Robot {
            direction: Direction::East,
            location: robot.location,
        };

        assert_eq!(robot, expected);
    }

    #[test]
    fn robot_moves_right() {
        let mut robot = Robot {
            location: Location { x: 0, y: 0 },
            direction: Direction::South,
        };
        robot.turn_right();

        let expected = Robot {
            direction: Direction::West,
            location: robot.location,
        };

        assert_eq!(robot, expected);
    }

    #[test]
    fn robot_follows_instructions() {
        let instruction = Instruction::Left;
        let mut robot = Robot {
            location: Location { x: 0, y: 0 },
            direction: Direction::South,
        };

        robot.follow(&vec![instruction]);

        let expected = Robot {
            direction: Direction::East,
            location: robot.location,
        };

        assert_eq!(robot, expected);
    }

    #[test]
    fn robot_follows_multiple_instructions() {
        let instructions = &vec![Instruction::Left, Instruction::Forward, Instruction::Right];
        let mut robot = Robot {
            location: Location { x: 1, y: 0 },
            direction: Direction::North,
        };

        let expected = Robot {
            direction: Direction::North,
            location: Location {
                x: robot.location.x - 1,
                y: robot.location.y,
            },
        };

        robot.follow(instructions);

        assert_eq!(robot, expected);
    }
}
