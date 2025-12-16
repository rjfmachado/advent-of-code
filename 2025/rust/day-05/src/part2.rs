use std::cmp;
use std::ops::Range;

use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let fresh_ingredient_ranges: Vec<Range<usize>> = input
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
                    Some(Range { start, end })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .flatten()
        .collect();

    let count: usize = fresh_ingredient_ranges
        .into_iter()
        .sorted_by(|a, b| a.start.cmp(&b.start)) //coalesce expects starting values in order
        .coalesce(|current, next| {
            //merges the ranges
            if next.start <= current.end {
                // Overlap found: Merge them
                // Return Ok(merged_item) to replace the pair with a single item
                Ok(current.start
                    ..cmp::max(current.end, next.end))
            } else {
                // No overlap: Keep them separate
                // Return Err((x, y)) to indicate the split point
                Err((current, next))
            }
        })
        .map(|range| range.end + 1 - range.start)
        .sum::<usize>();

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
12-18";
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
