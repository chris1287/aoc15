use nom::IResult;
use nom::character::complete::i32;
use nom::character::complete::alpha1;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use itertools::Itertools;

#[derive(Debug)]
pub struct Pair {
    pub a: String,
    pub b: String,
    pub happiness: i32,
}

fn parse_line(input: &str) -> IResult<&str, Pair>
{
    let (input, a) = alpha1(input)?;
    let (input, sign) = alt((
        map(tag(" would gain "), |_| 1),
        map(tag(" would lose "), |_| -1),
    ))(input)?;
    let (input, happiness) = i32(input)?;
    let (input, _) = tag(" happiness units by sitting next to ")(input)?;
    let (input, b) = alpha1(input)?;
    let (input, _) = tag(".")(input)?;

    Ok((input, Pair{
        a: a.to_string(),
        b: b.to_string(),
        happiness: sign * happiness,
    }))
}

fn parse(input: &str) -> (Vec<Pair>, Vec<String>) {
    let (_input, v) = separated_list1(
        line_ending,
        parse_line
    )(input).unwrap();

    let mut couples: Vec<Pair> = Vec::new();
    let mut persons: Vec<String> = Vec::new();
    for e in v {
        if !persons.contains(&e.a) {
            persons.push(e.a.clone());
        }
        if !persons.contains(&e.b) {
            persons.push(e.b.clone());
        }

        if let Some(x) = couples.iter_mut().find(|c|
            c.a == e.a && c.b == e.b ||
            c.b == e.a && c.a == e.b
        ) {
            x.happiness += e.happiness;
        } else {
            couples.push(e);
        }
    }

    let me = "me";
    for p in &persons {
        couples.push(Pair{
            a: me.to_string(),
            b: p.to_string(),
            happiness: 0
        })
    }
    persons.push(me.to_string());

    (couples, persons)
}

pub fn solve(s: &str) -> i32 {
    let (couples, persons) = parse(s);

    let mut max_happiness = 0;
    for perm in persons.iter().permutations(persons.len()).unique() {
        let mut table = perm.clone();
        table.push(perm[0]);
        let n = table.windows(2).fold(0, |acc, w|{
            let a = w[0];
            let b = w[1];
            couples.iter().find(|c|{
                c.a == *a && c.b == *b ||
                c.b == *a && c.a == *b
            }).expect("all couples should exist").happiness + acc
        });
        if n > max_happiness {
            max_happiness = n;
        }
    }

    max_happiness
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
        assert_eq!(solve(&data), 668);
    }
}
