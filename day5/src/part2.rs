
pub fn solve(s: &str) -> usize  {
    s.lines().filter(|s|{
        let mut pairs = false;
        for i in 0..s.len()-1 {
            let j = i+1;
            let left = &s[..i];
            let right = &s[j+1..];
            let mid = &s[i..j+1];
            if left.contains(mid) || right.contains(mid) {
                pairs = true;
                break;
            }
        }

        let triplets = s.as_bytes().windows(3).filter(|w|{
            w[0] == w[2]
        }).count() >= 1;

        pairs && triplets
    }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(solve("xxyxx"), 1);
        assert_eq!(solve("uurcxstgmygtbstg"), 0);
        assert_eq!(solve("ieodomkazucvgmuy"), 0);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 69);
    }
}
