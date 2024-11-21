struct Pos {
    x: i32,
    y: i32,
    count: usize,
}

pub fn solve(s: &str) -> usize {
    let mut xs = 0;
    let mut ys = 0;
    let mut xr = 0;
    let mut yr = 0;
    let mut positions: Vec<Pos> = vec!(Pos{
        x: xs,
        y: ys,
        count: 2
    });
    for (n, c) in s.chars().enumerate() {
        let (x, y) = {
            if n%2 == 0 {
                (&mut xs, &mut ys)
            } else {
                (&mut xr, &mut yr)
            }
        };
        match c {
            '^' => { *y = *y+1 },
            'v' => { *y = *y-1 },
            '>' => { *x = *x+1 },
            '<' => { *x = *x-1 },
            _ => {}
        };
        if let Some(pos) = positions.iter_mut().find(|p| {
            (**p).x == *x && (**p).y == *y
        }) {
            pos.count += 1
        } else {
            positions.push(Pos{
                x: *x,
                y: *y,
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
        assert_eq!(solve("^v"), 3);
        assert_eq!(solve("^>v<"), 3);
        assert_eq!(solve("^v^v^v^v^v"), 11);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 2341);
    }
}
