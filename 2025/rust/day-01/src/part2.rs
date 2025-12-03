const DIAL_SIZE: i32 = 100;
const START_DIAL_POSITION: i32 = 50;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // Track how many times we land on position 0 while walking the ring,
    // including crossings caused by oversized step counts.
    let mut count: i32 = 0;
    // Positions live on a 0-99 ring; start in the middle at 50.
    let mut position: i32 = START_DIAL_POSITION;
    let _orders: Vec<_> = input
        .lines()
        .map(|line| {
            // Parse the line into a direction (L/R) and a signed step amount.
            let mut line = line.chars();
            if let Some(direction_char) = line.next() {
                let mut value: i32 =
                    line.as_str().parse().unwrap();

                // If the step count spans multiple full laps,
                // pre-count the zero crossings those laps guarantee.
                if value > DIAL_SIZE {
                    let multiple =
                        (value / DIAL_SIZE).abs();
                    count = count + multiple;
                    value = value % DIAL_SIZE;
                }

                let command: Direction =
                    match direction_char {
                        'L' => Direction::Left(value),
                        'R' => Direction::Right(value),
                        _ => panic!("Bad input"),
                    };
                //dbg!(&command);
                // Move around the ring, wrapping via modulo arithmetic.
                match command {
                    Direction::Left(i) => {
                        // Crosses zero when stepping left from a positive position.
                        if position - i < 0 && position != 0
                        {
                            count += 1
                        }
                        position = (position - i)
                            .rem_euclid(DIAL_SIZE)
                    }
                    Direction::Right(i) => {
                        // Crosses zero when stepping right beyond the upper bound.
                        if position + i > DIAL_SIZE {
                            count += 1
                        }
                        position = (position + i)
                            .rem_euclid(DIAL_SIZE)
                    }
                };
            }

            // Count each visit to position 0 after applying the move.
            if position == 0 {
                count += 1
            }
            //dbg!(&count);
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
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
