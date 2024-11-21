use nom::IResult;
use nom::character::complete::i32;
use nom::character::complete::alpha1;
use nom::character::complete::line_ending;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;

#[derive(Debug)]
pub struct Reindeer {
    pub name: String,
    pub speed: i32,
    pub fly_time: i32,
    pub rest: i32,
}

fn parse_line(input: &str) -> IResult<&str, Reindeer>
{
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(" can fly ")(input)?;
    let (input, speed) = i32(input)?;
    let (input, _) = tag(" km/s for ")(input)?;
    let (input, fly_time) = i32(input)?;
    let (input, _) = tag(" seconds, but then must rest for ")(input)?;
    let (input, rest) = i32(input)?;
    let (input, _) = tag(" seconds.")(input)?;
    Ok((input, Reindeer{
        name: name.to_string(),
        speed,
        fly_time,
        rest,
    }))
}

fn parse(input: &str) -> Vec<Reindeer> {
    let (_, reindeers) = separated_list1(
        line_ending,
        parse_line
    )(input).unwrap();
    reindeers
}

pub fn solve(s: &str, t: i32) -> i32 {
    let reindeers = parse(s);
    let mut max_d = 0;
    for r in reindeers {
        let mut t = t;
        let mut d = 0;
        while t > 0 {
            let time_avail = std::cmp::min(t, r.fly_time);
            d += r.speed * time_avail;
            t -= r.fly_time;
            t -= r.rest;
        }
        if d > max_d {
            max_d = d;
        }
    }
    max_d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        assert_eq!(solve(&data, 1000), 1120);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data, 2503), 2660);
    }
}
