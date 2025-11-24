use std::collections::HashMap;
use std::ops::{Add, Sub};

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", run(input.trim()))
}

fn run(input: &str) -> String {
    let keypad: HashMap<(i32, i32), &str> = HashMap::from([
        ((0, 2), "1"),
        ((-1, 1), "2"),
        ((0, 1), "3"),
        ((1, 1), "4"),
        ((-2, 0), "5"),
        ((-1, 0), "6"),
        ((0, 0), "7"),
        ((1, 0), "8"),
        ((2, 0), "9"),
        ((-1, -1), "A"),
        ((0, -1), "B"),
        ((1, -1), "C"),
        ((0, -2), "D"),
    ]);

    let mut position: (i32, i32) = (-2, 0);
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code() {
        assert_eq!(
            "5DB3",
            run("ULL
RRDDD
LURDL
UUUUD")
        );
    }
}
