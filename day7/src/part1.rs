use nom:: {
    IResult,
    bytes::complete::tag,
    multi::separated_list1,
    character::complete::line_ending,
    branch::alt,
    character::complete::u16,
    character::complete::alpha1,
    combinator::map,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Var {
    Value(u16),
    Name(String),
    None,
}

#[derive(Debug)]
struct Operation {
    a: Var,
    b: Var,
    x: Var,
    o: Operator,
}

#[derive(Debug)]
pub enum Operator {
    And,
    Or,
    Ls,
    Rs,
    Not,
    Eq,
}

fn parse_var(input: &str) -> IResult<&str, Var> {
    let (input, x) = alt((
        map(alpha1, |s: &str| Var::Name(s.to_string())),
        map(u16, |n| Var::Value(n)),
    ))(input)?;
    Ok((input, x))
}

fn parse_and(input: &str) -> IResult<&str, Operation> {
    let (input, a) = parse_var(input)?;
    let (input, _) = tag(" AND ")(input)?;
    let (input, b) = parse_var(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, x) = parse_var(input)?;
    Ok((input, Operation {
        a,
        b,
        x,
        o: Operator::And
    }))
}

fn parse_or(input: &str) -> IResult<&str, Operation> {
    let (input, a) = parse_var(input)?;
    let (input, _) = tag(" OR ")(input)?;
    let (input, b) = parse_var(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, x) = parse_var(input)?;
    Ok((input, Operation {
        a,
        b,
        x,
        o: Operator::Or
    }))
}

fn parse_ls(input: &str) -> IResult<&str, Operation> {
    let (input, a) = parse_var(input)?;
    let (input, _) = tag(" LSHIFT ")(input)?;
    let (input, b) = parse_var(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, x) = parse_var(input)?;
    Ok((input, Operation {
        a,
        b,
        x,
        o: Operator::Ls
    }))
}

fn parse_rs(input: &str) -> IResult<&str, Operation> {
    let (input, a) = parse_var(input)?;
    let (input, _) = tag(" RSHIFT ")(input)?;
    let (input, b) = parse_var(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, x) = parse_var(input)?;
    Ok((input, Operation {
        a,
        b,
        x,
        o: Operator::Rs
    }))
}

fn parse_not(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("NOT ")(input)?;
    let (input, a) = parse_var(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, x) = parse_var(input)?;
    Ok((input, Operation {
        a,
        b: Var::None,
        x,
        o: Operator::Not
    }))
}

fn parse_eq(input: &str) -> IResult<&str, Operation> {
    let (input, a) = parse_var(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, x) = parse_var(input)?;
    Ok((input, Operation {
        a,
        b: Var::None,
        x,
        o: Operator::Eq
    }))
}

fn parse(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, operations) = separated_list1(
        line_ending,
        alt((
            parse_and,
            parse_or,
            parse_ls,
            parse_rs,
            parse_not,
            parse_eq,
        ))
    )(input)?;
    Ok((input, operations))
}

pub fn solve(s: &str, name: &str) -> u16 {
    let(_input, operations) = parse(s).unwrap();
    let name = name.to_string();

    let mut solutions = std::collections::HashMap::<&String, u16>::new();

    // Infinite loop assumption: a solution exists
    loop {
        for operation in &operations {
            let x = match &operation.x {
                Var::Name(n) => n,
                _ => unreachable!(),
            };
            if solutions.contains_key(x) {
                continue;
            }
            match operation.o {
                Operator::Eq => {
                    match &operation.a {
                        Var::Name(n) => {
                            if let Some(n) = solutions.get(&n) {
                                solutions.insert(x, *n);
                            }
                        },
                        Var::Value(n) => {
                            solutions.insert(x, *n);
                        },
                        _ => unreachable!(),
                    }
                },
                Operator::Not => {
                    match &operation.a {
                        Var::Name(n) => {
                            if let Some(n) = solutions.get(&n) {
                                solutions.insert(x, !(*n));
                            }
                        },
                        Var::Value(n) => {
                            solutions.insert(x, *n);
                        },
                        _ => unreachable!(),
                    }
                },
                Operator::And => {
                    match (&operation.a,&operation.b) {
                        (Var::Value(a),Var::Value(b)) => {
                            solutions.insert(x, a&b);
                        },
                        (Var::Name(a),Var::Name(b)) => {
                            if let (Some(a), Some(b)) = (solutions.get(&a),solutions.get(&b))  {
                                solutions.insert(x, a&b);
                            }
                        },
                        (Var::Name(a),Var::Value(b)) => {
                            if let Some(a) = solutions.get(&a) {
                                solutions.insert(x, a&b);
                            }
                        },
                        (Var::Value(a),Var::Name(b)) => {
                            if let Some(b) = solutions.get(&b) {
                                solutions.insert(x, a&b);
                            }
                        },
                        _ => unreachable!(),
                    }
                },
                Operator::Or => {
                    match (&operation.a,&operation.b) {
                        (Var::Value(a),Var::Value(b)) => {
                            solutions.insert(x, a|b);
                        },
                        (Var::Name(a),Var::Name(b)) => {
                            if let (Some(a), Some(b)) = (solutions.get(&a),solutions.get(&b))  {
                                solutions.insert(x, a|b);
                            }
                        },
                        (Var::Name(a),Var::Value(b)) => {
                            if let Some(a) = solutions.get(&a) {
                                solutions.insert(x, a|b);
                            }
                        },
                        (Var::Value(a),Var::Name(b)) => {
                            if let Some(b) = solutions.get(&b) {
                                solutions.insert(x, a|b);
                            }
                        },
                        _ => unreachable!(),
                    }
                },
                Operator::Ls => {
                    match (&operation.a,&operation.b) {
                        (Var::Value(a),Var::Value(b)) => {
                            solutions.insert(x, a<<b);
                        },
                        (Var::Name(a),Var::Name(b)) => {
                            if let (Some(a), Some(b)) = (solutions.get(&a),solutions.get(&b))  {
                                solutions.insert(x, a<<b);
                            }
                        },
                        (Var::Name(a),Var::Value(b)) => {
                            if let Some(a) = solutions.get(&a) {
                                solutions.insert(x, a<<b);
                            }
                        },
                        (Var::Value(a),Var::Name(b)) => {
                            if let Some(b) = solutions.get(&b) {
                                solutions.insert(x, a<<b);
                            }
                        },
                        _ => unreachable!(),
                    }
                },
                Operator::Rs => {
                    match (&operation.a,&operation.b) {
                        (Var::Value(a),Var::Value(b)) => {
                            solutions.insert(x, a>>b);
                        },
                        (Var::Name(a),Var::Name(b)) => {
                            if let (Some(a), Some(b)) = (solutions.get(&a),solutions.get(&b))  {
                                solutions.insert(x, a>>b);
                            }
                        },
                        (Var::Name(a),Var::Value(b)) => {
                            if let Some(a) = solutions.get(&a) {
                                solutions.insert(x, a>>b);
                            }
                        },
                        (Var::Value(a),Var::Name(b)) => {
                            if let Some(b) = solutions.get(&b) {
                                solutions.insert(x, a>>b);
                            }
                        },
                        _ => unreachable!(),
                    }
                },
            }
        }

        if let Some(s) = solutions.get(&name) {
            return *s;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
        assert_eq!(solve(&data, "d"), 72);
        assert_eq!(solve(&data, "e"), 507);
        assert_eq!(solve(&data, "f"), 492);
        assert_eq!(solve(&data, "g"), 114);
        assert_eq!(solve(&data, "h"), 65412);
        assert_eq!(solve(&data, "i"), 65079);
        assert_eq!(solve(&data, "x"), 123);
        assert_eq!(solve(&data, "y"), 456);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data, "a"), 16076);
    }
}
