use std::collections::HashMap;
use std::ops::{Add, Sub};

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", run(input.trim()))
}

fn run(input: &str) -> i32 {
    let keypad: HashMap<(i32, i32), i32> = HashMap::from([
        ((-1, 1), 1),
        ((0, 1), 2),
        ((1, 1), 3),
        ((-1, 0), 4),
        ((0, 0), 5),
        ((1, 0), 6),
        ((-1, -1), 7),
        ((0, -1), 8),
        ((1, -1), 9),
    ]);

    let mut position: (i32, i32) = (0, 0);
    let mut next: (i32, i32) = (0, 0);

    let code = input
        .lines()
        .map(|line| {
            line.chars().fold(keypad.get(&position), |acc, udlr| {
                match udlr {
                    'U' => next = (position.0, position.1.add(1)),
                    'D' => next = (position.0, position.1.sub(1)),
                    'L' => next = (position.0.sub(1), position.1),
                    'R' => next = (position.0.add(1), position.1),
                    _ => panic!(),
                }

                if keypad.contains_key(&next) {
                    position = next;
                    keypad.get(&next)
                } else {
                    acc
                }
            })
        })
        .collect::<Vec<_>>();

    code.into_iter()
        .fold(String::new(), |acc, key| acc + &key.unwrap().to_string())
        .parse::<i32>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code() {
        assert_eq!(
            1985,
            run("ULL
RRDDD
LURDL
UUUUD")
        );
    }
}
