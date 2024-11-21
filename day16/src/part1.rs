use nom::IResult;
use nom::character::complete::u64;
use nom::character::complete::alpha1;
use nom::character::complete::line_ending;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;

#[derive(Debug, Eq, PartialEq)]
pub enum Compound {
    Children(u64),
    Cats(u64),
    Samoyeds(u64),
    Pomeranians(u64),
    Akitas(u64),
    Vizslas(u64),
    Goldfish(u64),
    Trees(u64),
    Cars(u64),
    Perfumes(u64),
}

#[derive(Debug)]
pub struct Sue {
    pub name: u64,
    pub compounds: Vec<Compound>,
}

fn parse_compound(input: &str) -> IResult<&str, Compound> {
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, qty) = u64(input)?;
    Ok((input, match name {
        "children" => Compound::Children(qty),
        "cats" => Compound::Cats(qty),
        "samoyeds" => Compound::Samoyeds(qty),
        "pomeranians" => Compound::Pomeranians(qty),
        "akitas" => Compound::Akitas(qty),
        "vizslas" => Compound::Vizslas(qty),
        "goldfish" => Compound::Goldfish(qty),
        "trees" => Compound::Trees(qty),
        "cars" => Compound::Cars(qty),
        "perfumes" => Compound::Perfumes(qty),
        _ => unreachable!(),
    }))
}

fn parse_line(input: &str) -> IResult<&str, Sue>
{
    let (input, _) = tag("Sue ")(input)?;
    let (input, name) = u64(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, compounds) = separated_list1(
        tag(", "),
        parse_compound
    )(input)?;

    Ok((input, Sue{
        name,
        compounds
    }))
}

fn parse(input: &str) -> Vec<Sue> {
    let (_, v) = separated_list1(
        line_ending,
        parse_line
    )(input).unwrap();
    v
}

pub fn fitness(a: &Sue, b: &Sue) -> u64 {
    let mut matching = 0;
    for compound in &a.compounds {
        matching += b.compounds.iter().filter(|c| *c == compound).count();
    }
    matching as u64
}

pub fn solve(s: &str) -> u64 {
    let v = parse(s);

    let perfect = Sue {
        name: 0,
        compounds: vec![
            Compound::Children(3),
            Compound::Cats(7),
            Compound::Samoyeds(2),
            Compound::Pomeranians(3),
            Compound::Akitas(0),
            Compound::Vizslas(0),
            Compound::Goldfish(5),
            Compound::Trees(3),
            Compound::Cars(2),
            Compound::Perfumes(1),
        ]
    };

    let mut max = 0;
    let mut idx = 0;
    for sue in v {
        let f = fitness(&perfect, &sue);
        if f > max {
            max = f;
            idx = sue.name;
        }
    }
    idx
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
        assert_eq!(solve(&data), 40);
    }
}
