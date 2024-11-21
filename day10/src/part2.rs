use itertools::Itertools;

pub fn count(s: &str) -> String {
    let mut res = String::new();
    let v = s.chars().dedup_with_count();
    for (count, c) in v {
        res.push_str(format!("{}{}", count, c).as_str());
    }
    res
}

pub fn solve(s: &str, n: usize) -> usize {
    let mut s = s.to_string();
    for _ in 0..n {
        s = count(&s);
    }
    s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "1";
        assert_eq!(solve(&data, 5), 6);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        let data = data.trim();
        assert_eq!(solve(&data, 50), 3579328);
    }
}
