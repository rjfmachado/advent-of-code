use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let output_joltage: u64 = 0;
    let battery_banks: Vec<&str> = input.lines().collect();
    dbg!(&battery_banks);

    let max_joltage_per_bank: Vec<_> = battery_banks
        .into_iter()
        .map(|bank| {
            let trim_bank = &bank[0..bank.len() - 1]; //remove last as it wont ever be used as the major pair
            trim_bank.chars().enumerate().max_set_by_key(
                |&(_index, joltage)| joltage,
            );
            dbg!(&trim_bank);
        })
        .collect();
    dbg!(&max_joltage_per_bank);
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
