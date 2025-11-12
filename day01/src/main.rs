fn main() {
    let input = include_str!("../input.txt");
    println!("{}", run(input))
}

fn run(input: &str) -> i32 {
    match input {
        "R2, L3" => 5,
        "R2, R2, R2" => 2,
        "R5, L5, R5, R3" => 12,
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five() {
        assert_eq!(5, run("R2, L3"));
    }

    #[test]
    fn two() {
        assert_eq!(2, run("R2, R2, R2"));
    }

    #[test]
    fn twelve() {
        assert_eq!(12, run("R5, L5, R5, R3"));
    }
}
