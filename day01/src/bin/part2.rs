#[derive(Debug)]
struct Me {
    x: i32,
    y: i32,
    direction: Direction,
    visited: Vec<(i32, i32)>,
}

impl Me {
    fn r#move(self: &mut Me, instruction: (&str, &str)) -> Option<(i32, i32)> {
        self.direction = match instruction.0 {
            "L" => match self.direction {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            },
            "R" => match self.direction {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
            _ => panic!(),
        };

        let distance = instruction.1.parse::<i32>().unwrap();
        match self.direction {
            Direction::North => {
                for _ in 0..distance {
                    self.y += 1;
                    if self.visited.contains(&(self.x, self.y)) {
                        return None;
                    }
                    self.visited.push((self.x, self.y));
                }
            }
            Direction::East => {
                for _ in 0..distance {
                    self.x += 1;
                    if self.visited.contains(&(self.x, self.y)) {
                        return None;
                    }
                    self.visited.push((self.x, self.y));
                }
            }
            Direction::South => {
                for _ in 0..distance {
                    self.y -= 1;
                    if self.visited.contains(&(self.x, self.y)) {
                        return None;
                    }
                    self.visited.push((self.x, self.y));
                }
            }
            Direction::West => {
                for _ in 0..distance {
                    self.x -= 1;
                    if self.visited.contains(&(self.x, self.y)) {
                        return None;
                    }
                    self.visited.push((self.x, self.y));
                }
            }
        }

        Some((self.x, self.y))
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
    let input = include_str!("../../input.txt");
    println!("{}", run(input.trim()))
}

fn run(input: &str) -> i32 {
    let mut me = Me {
        x: 0,
        y: 0,
        direction: Direction::North,
        visited: vec![(0, 0)],
    };

    let _ = input
        .split(", ")
        .map_while(|instruction| me.r#move(instruction.split_at(1)))
        .collect::<Vec<_>>();

    me.x.abs() + me.y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twice() {
        assert_eq!(4, run("R8, R4, R4, R8"));
    }
}
