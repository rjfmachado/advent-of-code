const DIAL_SIZE: i32 = 100;
const START_DIAL_POSITION: i32 = 50;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // Number of times we land on position 0 after applying each instruction.
    let mut count: u16 = 0;
    // Walk a 0–99 ring, starting in the middle at position 50.
    let mut position: i32 = START_DIAL_POSITION;
    let _dial_commands: Vec<_> = input
        .lines()
        .map(|line| {
            // Split "L10" / "R3" into a direction and signed step count.
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

                // Apply the move and wrap via modulo so we stay on the ring.
                match command {
                    Direction::Left(i) => {
                        position = (position - i)
                            .rem_euclid(DIAL_SIZE)
                    }
                    Direction::Right(i) => {
                        position = (position + i)
                            .rem_euclid(DIAL_SIZE)
                    }
                };
            }

            // Record any landing on position 0 after this instruction.
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
