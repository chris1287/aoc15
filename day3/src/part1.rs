struct Pos {
    x: i32,
    y: i32,
    count: usize,
}

pub fn solve(s: &str) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut positions: Vec<Pos> = vec!(Pos{
        x,
        y,
        count: 1
    });
    for c in s.chars() {
        match c {
            '^' => {y = y+1},
            'v' => {y = y-1},
            '>' => {x = x+1},
            '<' => {x = x-1},
            _ => {}
        };
        if let Some(pos) = positions.iter_mut().find(|p| {
            (**p).x == x && (**p).y == y
        }) {
            pos.count += 1
        } else {
            positions.push(Pos{
                x,
                y,
                count: 1
            })
        }
    }
    positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve(">"), 2);
        assert_eq!(solve("^>v<"), 4);
        assert_eq!(solve("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 2081);
    }
}
