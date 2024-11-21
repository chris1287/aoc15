use nom::IResult;
use nom::character::complete::i64;
use nom::bytes::complete::is_not;

fn count(input: &str) -> IResult<&str, i64> {
    let mut sum = 0;
    let mut input = input;
    loop {
        if input.is_empty() {
            break;
        }
        (input, _) = is_not("-0123456789")(input)?;
        if input.is_empty() {
            break;
        }
        let n;
        (input, n) = i64(input)?;
        sum += n;
    }
    Ok((input, sum))
}

pub fn solve(s: &str) -> i64 {
    let (input, sum) = count(s).unwrap();
    assert!(input.is_empty());
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve(r#"[1,2,3]"#), 6);
        assert_eq!(solve(r#"{"a":2,"b":4}"#), 6);
        assert_eq!(solve(r#"[[[3]]]"#), 3);
        assert_eq!(solve(r#"{"a":{"b":4},"c":-1}"#), 3);
        assert_eq!(solve(r#"{"a":[-1,1]}"#), 0);
        assert_eq!(solve(r#"[-1,{"a":1}]"#), 0);
        assert_eq!(solve(r#"[]"#), 0);
        assert_eq!(solve(r#"{}"#), 0);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 191164);
    }
}
