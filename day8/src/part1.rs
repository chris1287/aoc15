pub fn solve(s: &str) -> usize {
    let mut tot = 0;
    for s in s.lines() {
        let s = s.as_bytes();
        let len = s.len();
        let mut i = 0;
        let mut count = 0;
        while i < len {
            if s[i] == b'\\' {
                if i+1 >= len {
                    panic!("invalid string");
                }
                if s[i+1] == b'"' {
                    i += 2;
                    count += 1;
                } else if s[i+1] == b'\\' {
                    i += 2;
                    count += 1;
                } else if s[i+1] == b'x' {
                    if i+3 >= len {
                        panic!("invalid string");
                    }
                    i += 4;
                    count += 1;
                } else {
                    panic!("invalid string")
                }
            } else if s[i] == b'\"' {
                i+= 1;
                count += 0;
            } else {
                i += 1;
                count += 1;
            }
        }
        tot += len-count;
    }
    tot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve(r#""""#), 2);
        assert_eq!(solve(r#""abc""#), 2);
        assert_eq!(solve(r#""aaa\"aaa""#), 3);
        assert_eq!(solve(r#""\x27""#), 5);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 1350);
    }
}
