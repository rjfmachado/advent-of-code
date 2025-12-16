use std::ops::RangeInclusive;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut count: usize = 0;
    let mut ingredient_ids: Vec<usize> = Vec::new();
    let mut fresh_ingredient_ranges: Vec<
        RangeInclusive<usize>,
    > = Vec::new();

    let _x: Vec<_> = input
        .lines()
        .map(|line| {
            if !line.is_empty() {
                if line.contains('-') {
                    let start: usize = line
                        .split_once('-')
                        .unwrap()
                        .0
                        .to_owned()
                        .parse()
                        .unwrap();
                    let end: usize = line
                        .split_once('-')
                        .unwrap()
                        .1
                        .to_owned()
                        .parse()
                        .unwrap();
                    fresh_ingredient_ranges
                        .push(start..=end);
                } else {
                    ingredient_ids
                        .push(line.parse().unwrap())
                }
            } else {
            }
        })
        .collect();
    for ingredient in ingredient_ids {
        let mut fresh = false;
        for range in &fresh_ingredient_ranges {
            if range.contains(&ingredient) {
                if fresh == false {
                    count += 1;
                    fresh = true;
                }
            }
        }
    }
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
