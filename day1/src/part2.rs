pub fn solve(s: &str) -> i32 {
    let mut acc = 0;
    for(pos, c) in s.chars().enumerate() {
        acc = match c {
            '(' => acc + 1, 
            ')' => acc -1,
            _ => acc
        };
        if acc < 0 {
            return pos as i32 + 1;
        }
    }
    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve(")"), 1);
        assert_eq!(solve("()())"), 5);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 1797);
    }
}
