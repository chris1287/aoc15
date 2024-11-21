
pub fn solve(s: &str) -> usize  {
    s.lines().filter(|s|{
        let vowels = s.chars().filter(|x| {
            match x {
                'a'|'e'|'i'|'o'|'u' => true,
                _ => false
            }
        }).count() >= 3;

        let twin = s.as_bytes().windows(2).find(|w| w[0] == w[1]).is_some();

        let undesired = s.as_bytes().windows(2).find(|w| {
            match w {
                [b'a', b'b'] => true,
                [b'c', b'd'] => true,
                [b'p', b'q'] => true,
                [b'x', b'y'] => true,
                _ => false
            }
        }).is_none();

        vowels && twin && undesired

    }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve("ugknbfddgicrmopn"), 1);
        assert_eq!(solve("aaa"), 1);
        assert_eq!(solve("jchzalrnumimnmhp"), 0);
        assert_eq!(solve("haegwjzuvuyypxyu"), 0);
        assert_eq!(solve("dvszwmarrgswjxmb"), 0);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 238);
    }
}
