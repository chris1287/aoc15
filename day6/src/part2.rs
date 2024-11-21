use nom:: {
    IResult,
    bytes::complete::tag,
    multi::separated_list1,
    character::complete::line_ending,
    branch::alt,
    combinator::map,
    character::complete::u32,
    sequence::separated_pair,
};
use grid::Grid;

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub enum Instruction {
    On,
    Off,
    Toggle,
}

#[derive(Debug)]
pub struct Cmd {
    pub instruction: Instruction,
    pub start: Point,
    pub end: Point,
}

pub fn parse_line(input: &str) -> IResult<&str, Cmd> {
    let (input, instruction) = alt((
        map(tag("turn on "), |_| Instruction::On),
        map(tag("turn off "), |_| Instruction::Off),
        map(tag("toggle "), |_| Instruction::Toggle),
    ))(input)?;
    let (input, (x1, y1)) = separated_pair(u32, tag(","), u32)(input)?;
    let (input, _) = tag(" through ")(input)?;
    let (input, (x2, y2)) = separated_pair(u32, tag(","), u32)(input)?;
    Ok((input, Cmd{
        instruction,
        start: Point {
            x: x1 as usize,
            y: y1 as usize,
        },
        end: Point {
            x: x2 as usize,
            y: y2 as usize,
        }
    }))
}

fn parse(input: &str) -> IResult<&str, Vec<Cmd>> {
    let (input, v) = separated_list1(
        line_ending,
        parse_line
    )(input)?;
    Ok((input, v))
}

pub fn solve(s: &str) -> usize  {
    let (_input, commands) = parse(s).unwrap();
    let mut grid = Grid::init(1000, 1000, 0);
    for command in commands {
        for x in command.start.x..=command.end.x {
            for y in command.start.y..=command.end.y {
                match command.instruction {
                    Instruction::On => grid[(x, y)] = grid[(x, y)]+1,
                    Instruction::Off => {
                        if grid[(x, y)] > 0 {
                            grid[(x, y)] = grid[(x, y)]-1;
                        }
                    },
                    Instruction::Toggle => grid[(x, y)] = grid[(x, y)]+2,
                }
            }
        }
    }
    grid.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve("turn on 0,0 through 0,0"), 1);
        assert_eq!(solve("toggle 0,0 through 999,999"), 2000000);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 17836115);
    }
}
