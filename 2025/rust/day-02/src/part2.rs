#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut sum: u64 = 0;
    let _range_list: Vec<_> = input
        .split(",")
        .map(|range| {
            //dbg!(&range);
            let id_pair = range.split_once("-").unwrap();
            for id in id_pair.0.parse::<u64>().unwrap()
                ..=id_pair.1.parse::<u64>().unwrap()
            {
                let id_string = id.to_string();
                let id_len = id_string.len();
                let mut invalid = false;

                for match_size in 1..id_len {
                    let match_pattern =
                        &id_string[0..match_size];

                    let match_count = id_string
                        .matches(match_pattern)
                        .count();
                    // dbg!(
                    //     &id,
                    //     match_count,
                    //     match_size,
                    //     match_pattern
                    // );
                    if match_count * match_size == id_len
                        && match_count >= 2
                    {
                        invalid = true;
                        break;
                    };
                }

                if invalid {
                    sum += id;
                    //dbg!(id, sum);
                }
            }
        })
        .collect();
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("4174379265", process(input)?);
        Ok(())
    }
}
