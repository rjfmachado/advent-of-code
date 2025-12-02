#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // Track how many times we land exactly on position 0 while walking the ring.
    let mut count: u16 = 0;
    // Positions live on a 0–99 ring; start in the middle at 50.
    let mut position: i32 = 50;
    let _orders: Vec<_> = input
        .lines()
        .map(|line| {
            // Parse the line into a direction (L/R) and a signed step amount.
            let mut line = line.chars();
            if let Some(direction_char) = line.next() {
                let value: i32 =
                    line.as_str().parse().unwrap();

                let command: Direction =
                    match direction_char {
                        'L' => Direction::Left(value),
                        'R' => Direction::Right(value),
                        _ => panic!("Bad input"),
                    };

                // Move around the ring, wrapping via modulo arithmetic.
                match command {
                    Direction::Left(i) => {
                        position =
                            (position - i).rem_euclid(100)
                    }
                    Direction::Right(i) => {
                        position =
                            (position + i).rem_euclid(100)
                    }
                };
            }

            // Count each visit to position 0 after applying the move.
            if position == 0 {
                count += 1
            }
        })
        .collect();
    Ok(count.to_string())
}

#[derive(Debug)]
enum Direction {
    Left(i32),
    Right(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
