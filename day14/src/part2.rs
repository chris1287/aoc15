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
    pub points: i32,
    pub travelled_distance: i32,
    pub remain_fly_time: i32,
    pub rested_for: i32,
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
        points: 0,
        travelled_distance: 0,
        remain_fly_time: fly_time,
        rested_for: 0,
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
    let mut reindeers = parse(s);
    let mut max_d = 0;
    let mut max_p = 0;

    for _tick in 0..t {
        for r in &mut reindeers {
            if r.remain_fly_time > 0 {
                r.remain_fly_time -= 1;
                r.travelled_distance += r.speed;
            } else if r.rested_for < r.rest {
                r.rested_for += 1;
            } else {
                r.remain_fly_time = r.fly_time - 1;
                r.travelled_distance += r.speed;
                r.rested_for = 0;
            }
            if r.travelled_distance > max_d {
                max_d = r.travelled_distance;
            }
        }
        for r in &mut reindeers {
            if r.travelled_distance == max_d {
                r.points += 1;
            }
        }
    }

    for r in &mut reindeers {
        if r.points > max_p {
            max_p = r.points;
        }
    }

    max_p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        assert_eq!(solve(&data, 1000), 689);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data, 2503), 1256);
    }
}
