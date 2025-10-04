use rand::distr::{Distribution, StandardUniform};

pub type Point = (i32, i32);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Distribution<Direction> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        let index: u8 = rng.random_range(0..4);
        match index {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => unreachable!(),
        }
    }
}

struct RandomWalk {
    position: Point,
    max_length: i32,
    current_length: i32,
}

impl RandomWalk {
    pub fn new(length: i32) -> Self {
        // Create a new random walk type starting from the origin with a specified length
        Self {
            position: (0, 0),
            max_length: length,
            current_length: 0,
        }
    }
}

impl Iterator for RandomWalk {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_length == self.max_length {
            return None;
        }
        let next_direction = rand::random();
        match next_direction {
            Direction::Up => self.position.1 += 1,
            Direction::Down => self.position.1 += -1,
            Direction::Left => self.position.0 += -1,
            Direction::Right => self.position.0 += 1,
        }
        self.current_length += 1;
        return Some(self.position);
    }
}
