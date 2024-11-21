use nom::IResult;
use nom::character::complete::i64;
use nom::character::complete::alpha1;
use nom::character::complete::line_ending;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use std::collections::HashMap;
use std::hash::{Hash, Hasher, DefaultHasher};

#[derive(Debug, Eq, PartialEq)]
pub struct Ingredient {
    pub capacity: i64,
    pub durability: i64,
    pub flavor: i64,
    pub texture: i64,
    pub calories: i64,
    pub qt: i64,
}

impl Hash for Ingredient {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.capacity.hash(state);
        self.durability.hash(state);
        self.flavor.hash(state);
        self.texture.hash(state);
        self.calories.hash(state);
        self.qt.hash(state);
    }
}

fn parse_line(input: &str) -> IResult<&str, Ingredient>
{
    let (input,
        (_name, _, capacity, _, durability, _, flavor, _, texture, _, calories)) = tuple((
        alpha1,
        tag(": capacity "),
        i64,
        tag(", durability "),
        i64,
        tag(", flavor "),
        i64,
        tag(", texture "),
        i64,
        tag(", calories "),
        i64,
    ))(input)?;
    Ok((input, Ingredient{
        capacity,
        durability,
        flavor,
        texture,
        calories,
        qt: 0,
    }))
}

fn parse(input: &str) -> Vec<Ingredient> {
    let (_, v) = separated_list1(
        line_ending,
        parse_line
    )(input).unwrap();
    v
}

fn recurse(v: &mut Vec<Ingredient>, depth: i64, memo: &mut HashMap<u64, i64>) -> i64 {
    if depth == 0 {
        let capacity: i64 = v.iter().map(|e|e.capacity*e.qt).sum();
        let durability: i64 = v.iter().map(|e|e.durability*e.qt).sum();
        let flavor: i64 = v.iter().map(|e|e.flavor*e.qt).sum();
        let texture: i64 = v.iter().map(|e|e.texture*e.qt).sum();
        if capacity<0 || durability<0 || flavor<0 || texture<0 {
            return 0;
        }
        return capacity * durability * flavor * texture;
    }
    let mut max = 0;
    for idx in 0..v.len() {
        v[idx].qt += 1;
        let mut hasher = DefaultHasher::new();
        v.hash(&mut hasher);
        let id = hasher.finish();
        let value = if let Some(n) = memo.get(&id) {
            *n
        } else {
            let value = recurse(v, depth-1, memo);
            memo.insert(id, value);
            value
        };
        v[idx].qt -= 1;
        max = std::cmp::max(max, value);
    }
    max
}

pub fn solve(s: &str) -> i64 {
    let mut v = parse(s);
    let mut memo = HashMap::new();
    recurse(&mut v, 100, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        assert_eq!(solve(&data), 62842880);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 13882464);
    }
}
