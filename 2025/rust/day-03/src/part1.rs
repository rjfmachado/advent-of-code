use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut output_joltage: u64 = 0;
    let battery_banks: Vec<&str> = input.lines().collect();
    //dbg!(&battery_banks);

    let max_joltage_per_bank: Vec<_> = battery_banks
        .iter()
        .map(|bank| {
            let trim_bank = &bank[0..bank.len() - 1]; //remove last as it wont ever be used as the major pair
            trim_bank.chars().enumerate().max_set_by_key(
                |&(_index, joltage)| joltage,
            )[0] // I only care about the first
        })
        .collect();
    //dbg!(&max_joltage_per_bank);
    //dbg!(battery_banks);

    let mut count = 0;
    let additional_max_battery: Vec<_> = battery_banks
        .iter()
        .map(|bank| {
            //dbg!(&max_joltage_per_bank[count].0);
            let y: usize =
                max_joltage_per_bank[count].0 + 1; //so we skip the max value from the 1st pass
            count += 1;
            bank[y..].chars().max().unwrap()
        })
        .collect();
    //dbg!(&additional_max_battery);
    for i in 0..battery_banks.len() {
        let x = format!(
            "{}{}",
            max_joltage_per_bank[i].1,
            additional_max_battery[i]
        );
        output_joltage += x.parse::<u64>().unwrap();
    }
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
