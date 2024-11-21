use petgraph::graphmap::UnGraphMap;
use petgraph::algo::simple_paths::all_simple_paths;

pub fn solve(s: &str) -> usize {
    let mut g = UnGraphMap::<&str, usize>::new();
    for line in s.lines() {
        let mut v = line.split(' ');
        let a = v.next().unwrap();
        let _ = v.next();
        let b = v.next().unwrap();
        let _ = v.next();
        let w: usize = v.next().unwrap().parse().unwrap();
        let a = g.add_node(a);
        let b = g.add_node(b);
        g.add_edge(a, b, w);
    }

    // println!("{:?}", petgraph::dot::Dot::new(&g));

    let mut min_cost = usize::MAX;
    let count = g.nodes().count();
    for start in g.nodes() {
        for end in g.nodes() {
            if start != end {
                let paths = all_simple_paths::<Vec<_>, _>(&g, start, end, count-2, None);
                for path in paths {
                    let mut cost = 0;
                    for t in path.windows(2) {
                        let a = t[0];
                        let b = t[1];
                        if let Some(w) = g.edge_weight(a, b) {
                            cost += w;
                        }
                    }
                    if cost < min_cost {
                        min_cost = cost;
                    }
                }
            }
        }
    }
    min_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
        assert_eq!(solve(&data), 605);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 251);
    }
}
