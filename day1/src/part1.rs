pub fn solve(s: &str) -> i32 {
    s.chars().fold(0, |acc, c|{
        match c {
            '(' => acc + 1, 
            ')' => acc -1,
            _ => acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve("(())"), 0);
        assert_eq!(solve("()()"), 0);
        assert_eq!(solve("((("), 3);
        assert_eq!(solve("(()(()("), 3);
        assert_eq!(solve("))((((("), 3);
        assert_eq!(solve("())"), -1);
        assert_eq!(solve("))("), -1);
        assert_eq!(solve(")))"), -3);
        assert_eq!(solve(")())())"), -3);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 280);
    }
}
