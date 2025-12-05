#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut count = 0;
    let ids = input.split(",").map(|id| {
        let mut invalid = false;
        let leading_zero
        if id[0..1] == *"0" {
            invalid = true
        }
    });
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
