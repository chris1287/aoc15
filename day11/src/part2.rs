fn add1(s: &str) -> String {
    let mut v = Vec::new();
    let mut change = true;
    for c in s.bytes().rev() {
        if change {
            if c != b'z' {
                v.push(c + 1);
                change = false;
            } else {
                v.push(b'a');
            }
        } else {
            v.push(c);
        }
    }
    v.reverse();
    String::from_utf8_lossy(&v).to_string()
}

fn remove_undesired(s: &str) -> (String, bool) {
    let mut v = Vec::new();
    let mut flip = false;
    for c in s.bytes() {
        if flip {
            v.push(b'a');
        } else {
            match c {
                b'i' => {
                    v.push(b'j');
                    flip = true;
                },
                b'o' => {
                    v.push(b'p');
                    flip = true;
                },
                b'l' => {
                    v.push(b'm');
                    flip = true;
                },
                c => {
                    v.push(c);
                }
            }
        }
    }
    (String::from_utf8_lossy(&v).to_string(), flip)
}

fn count_nooverlap(s: &str) -> usize {
    let mut count = 0;
    let mut i = 0;
    while i < s.len() - 1 {
        let bytes = s.as_bytes();
        if bytes[i] == bytes[i + 1] {
            count += 1;
            i += 2;
        } else {
            i += 1;
        }
    }
    count
}

pub fn solve(s: &str) -> String {
    let mut s = s.to_string();
    loop {
        let flip;
        (s, flip) = remove_undesired(&s);
        if !flip {
            s = add1(&s);
        }
        if s.as_bytes().windows(3).filter(|w| w[0]+1 == w[1] && w[1]+1 == w[2]).count() == 0 {
            continue;
        }
        if count_nooverlap(&s) < 2 {
            continue;
        }
        break;
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        let data = data.trim();
        assert_eq!(solve(&solve(&data)), "vzcaabcc");
    }
}
