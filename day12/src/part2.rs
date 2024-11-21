pub fn count(data: &serde_json::Value) -> Option<i64> {
    if let Some(s) = data.as_str() {
        if s == "red" {
            return None;
        } else {
            return Some(0);
        }
    }
    if let Some(n) = data.as_i64() {
        return Some(n);
    }
    if data.is_boolean() {
        return Some(0);
    }
    if let Some(v) = data.as_array() {
        let mut tot = 0;
        for e in v {
            if let Some(n) = count(&e) {
                tot += n;
            }
        }
        return Some(tot);
    }
    if let Some(v) = data.as_object() {
        let mut tot = 0;
        for (k, e) in v {
            if k == "red" || (e.is_string() && e.as_str().unwrap() == "red") {
                return Some(0);
            }
            if let Some(n) = count(e) {
                tot += n;
            }
        }
        return Some(tot);
    }
    unreachable!();
}

pub fn solve(s: &str) -> i64 {
    let data: serde_json::Value = serde_json::from_str(s).unwrap();
    count(&data).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve(r#"[1,2,3]"#), 6);
        assert_eq!(solve(r#"[1,{"c":"red","b":2},3]"#), 4);
        assert_eq!(solve(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
        assert_eq!(solve(r#"[1,"red",5]"#), 6);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 87842);
    }
}
