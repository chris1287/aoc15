pub fn solve(s: &str) -> usize {
    let mut tot = 0;
    for s in s.lines() {
        let s2 = s.replace(r#"\"#, r#"\\"#).replace(r#"""#, r#"\""#);
        tot += s2.len()-s.len()+2;
    }
    tot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve(r#""""#), 4);
        assert_eq!(solve(r#""abc""#), 4);
        assert_eq!(solve(r#""aaa\"aaa""#), 6);
        assert_eq!(solve(r#""\x27""#), 5);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 2085);
    }
}
