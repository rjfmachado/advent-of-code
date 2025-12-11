#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let output_joltage: u64 = 0;
    let _battery_banks: Vec<_> = input
        .lines()
        .map(|bank| {
            dbg!(&bank);
            let _pair: Vec<_> = bank
                .chars()
                .enumerate()
                .map(|t| {
                    dbg!(t);
                })
                .collect();
        })
        .collect();
    Ok(output_joltage.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!("357", process(input)?);
        Ok(())
    }
}
