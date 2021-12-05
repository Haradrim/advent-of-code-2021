use std::error::Error;

use utils::read_file;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_file("input.txt")?;

    let numbers = parse_input(input);

    println!("Part 1: {:?}", part_01(&numbers));

    println!("Part 2: {:?}", part_02(&numbers));

    Ok(())
}

fn parse_input(input: String) -> Vec<u32> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}

fn part_01(numbers: &Vec<u32>) -> usize {
    numbers.windows(2).filter(|pair| pair[0] < pair[1]).count()
}

fn part_02(numbers: &Vec<u32>) -> usize {
    let sums: Vec<u32> = numbers
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();

    sums.windows(2).filter(|pair| pair[0] < pair[1]).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_01() {
        let input = read_file("example.txt").unwrap();

        let numbers = parse_input(input);

        assert_eq!(part_01(&numbers), 7);
    }

    #[test]
    fn test_part_02() {
        let input = read_file("example.txt").unwrap();

        let numbers = parse_input(input);

        assert_eq!(part_02(&numbers), 5);
    }
}
