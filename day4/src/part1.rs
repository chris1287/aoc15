use md5;
use rayon::prelude::*;

pub fn solve(s: &str) -> usize  {
    if let Some(n) = (1..usize::MAX).into_par_iter().find_first(|n| {
        let v = format!("{}{}", s, *n);
        let dgst = format!("{:x}", md5::compute(&v));
        dgst.starts_with("00000")
    }) {
        return n;
    }
    panic!("no solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve("abcdef"), 609043);
        assert_eq!(solve("pqrstuv"), 1048970);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        let data = data.trim();
        assert_eq!(solve(data), 117946);
    }
}
