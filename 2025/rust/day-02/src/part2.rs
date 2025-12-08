#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut sum: u64 = 0;
    let _range_list: Vec<_> = input
        .split(",")
        .map(|range| {
            dbg!(&range);
            let id_pair = range.split_once("-").unwrap();
            for i in id_pair.0.parse::<u64>().unwrap()
                ..=id_pair.1.parse::<u64>().unwrap()
            {
                let s = i.to_string();
                let half = s.len() / 2;

                for j in 0..half {
                    if s[half..].contains(&s[..j]) {
                        sum += i;
                        dbg!(&s, sum);
                    }
                }
                // if &s[..half] == &s[half..] {
                //     sum += s.parse::<u64>().unwrap();
                //     dbg!(s, sum);
                // }
            }
        })
        .collect();
    dbg!(&sum);
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012";
        //let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("1227775554", process(input)?);
        Ok(())
    }
}
