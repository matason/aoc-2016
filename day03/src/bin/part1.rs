fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", run(input.trim()))
}

fn run(input: &str) -> i32 {
    let triangles = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|side| side.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .into_iter()
        .filter(|line| {
            line[0] + line[1] > line[2]
                && line[1] + line[2] > line[0]
                && line[2] + line[0] > line[1]
        })
        .collect::<Vec<_>>();

    triangles.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_this_works_its_going_to_be_brilliant() {
        assert_eq!(0, run("5 10 25"));
    }
}
