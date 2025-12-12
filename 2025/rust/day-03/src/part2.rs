use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    const WORKING_BATTERY_SIZE: usize = 12;
    let mut output_joltage: u64 = 0;
    //let mut working_battery_banks: Vec<String> = Vec::new();
    let battery_banks: Vec<&str> = input.lines().collect();

    for mut bank in battery_banks {
        //dbg!(&bank);
        let mut working_battery_bank = String::new();
        for i in (0..WORKING_BATTERY_SIZE).rev() {
            //reversed, so we count down the right side trim required
            //dbg!(&battery);
            let max = &bank[0..&bank.len() - i]
                .chars()
                .enumerate()
                .max_set_by_key(|&(_index, joltage)| {
                    joltage
                })[0];
            //dbg!(&max);
            working_battery_bank.push(max.1);
            bank = &bank[max.0 + 1..];
        }
        output_joltage +=
            &working_battery_bank.parse::<u64>().unwrap();
        //working_battery_banks.push(working_battery_bank);
    }
    //dbg!(&working_battery_banks);

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
        assert_eq!("3121910778619", process(input)?);
        Ok(())
    }
}
