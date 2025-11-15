#[derive(Debug)]
struct Me {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Me {
    fn r#move(self: &mut Me, instruction: (&str, &str)) {
        self.direction = match instruction.0 {
            "L" => match self.direction {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            }
            "R" => match self.direction {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            }
            _ => panic!()
        };

        let distance = instruction.1.parse::<i32>().unwrap();
        match self.direction {
            Direction::North => self.y += distance,
            Direction::East => self.x += distance,
            Direction::South => self.y -= distance,
            Direction::West => self.x -= distance,
        }
    }
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part1(input.trim()))
}

fn part1(input: &str) -> i32 {
    let mut me = Me {
        x: 0,
        y: 0,
        direction: Direction::North,
    };

    let _ = input.split(", ").map(|instruction| {
        me.r#move(instruction.split_at(1));
    }).collect::<Vec<_>>();

    ((0 - me.x) + (0 - me.y)).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five() {
        assert_eq!(5, part1("R2, L3"));
    }

    #[test]
    fn two() {
        assert_eq!(2, part1("R2, R2, R2"));
    }

    #[test]
    fn twelve() {
        assert_eq!(12, part1("R5, L5, R5, R3"));
    }
}
