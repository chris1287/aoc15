use nom:: {
    IResult,
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    character::complete::line_ending,
};

#[derive(Debug)]
struct Shape {
    l: usize,
    w: usize,
    h: usize,
}

fn parse_line(input: &str) -> IResult<&str, Shape> {
    let (input, l) = complete::i32(input)?;
    let (input, _) = tag("x")(input)?;
    let (input, w) = complete::i32(input)?;
    let (input, _) = tag("x")(input)?;
    let (input, h) = complete::i32(input)?;
    Ok((input, Shape {
        l: l as usize,
        w: w as usize,
        h: h as usize,
    }))
}

fn parse(input: &str) -> IResult<&str, Vec<Shape>> {
    let (input, v) = separated_list1(
        line_ending,
        parse_line
    )(input)?;
    Ok((input, v))
}

pub fn solve(s: &str) -> usize {
    let (_input, shapes) = parse(&s).unwrap();
    shapes.iter().fold(0, |acc, shape| {
        let a = 2*shape.l + 2*shape.w;
        let b = 2*shape.w + 2*shape.h;
        let c = 2*shape.h + 2*shape.l;
        let p = std::cmp::min(std::cmp::min(a, b), c);
        let v = shape.l * shape.w * shape.h;
        acc + p + v
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve("2x3x4"), 34);
        assert_eq!(solve("1x1x10"), 14);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 3737498);
    }
}
