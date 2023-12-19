//! # Day 19: Aplenty

use std::collections::HashMap;

#[derive(Debug)]
pub struct Part(u32, u32, u32, u32);

impl Part {
    fn parse(input: &str) -> Self {
        let parts = input
            .split(',')
            .map(|line| {
                line.chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap()
            })
            .collect::<Vec<_>>();

        Part(parts[0], parts[1], parts[2], parts[3])
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Filter<'a> {
    Send(&'a str),
    Less(usize, u32, &'a str),
    Greater(usize, u32, &'a str),
}

impl<'a> Filter<'a> {
    fn parse(input: &'a str) -> Self {
        match input.split_once(":") {
            Some((prefix, next)) => {
                let mut chars = prefix.chars();
                let c = chars.next().unwrap(); // the first char is the one we filter on, e.g. 'x', 'm', 'a' or 's'
                let c_idx = match c {
                    'x' => 0,
                    'm' => 1,
                    'a' => 2,
                    's' => 3,
                    _ => unreachable!(),
                };
                let s = chars.next().unwrap();
                let v = chars.collect::<String>().parse::<u32>().unwrap();

                match s {
                    '>' => Filter::Greater(c_idx, v, next),
                    '<' => Filter::Less(c_idx, v, next),
                    _ => unreachable!(),
                }
            }
            _ => Filter::Send(input),
        }
    }

    fn apply(&self, part: &Part) -> Option<&'a str> {
        match (part, self) {
            (Part(x, m, a, s), Filter::Greater(c, v, next)) => match c {
                0 if v < x => Some(next),
                1 if v < m => Some(next),
                2 if v < a => Some(next),
                3 if v < s => Some(next),
                _ => None,
            },
            (Part(x, m, a, s), Filter::Less(c, v, next)) => match c {
                0 if v > x => Some(next),
                1 if v > m => Some(next),
                2 if v > a => Some(next),
                3 if v > s => Some(next),
                _ => None,
            },
            (_, Filter::Send(next)) => Some(next),
        }
    }
}

pub fn input(raw: &str) -> (HashMap<String, Vec<Filter>>, Vec<Part>) {
    let (prefix, suffix) = raw.split_once("\n\n").unwrap();
    let mut workflows = HashMap::new();

    for line in prefix.lines() {
        let mut sections = line.split(['{', '}', ',']).filter(|l| !l.is_empty());
        let key = sections.next().unwrap().to_string();

        let mut filters = vec![];
        while let Some(next) = sections.next() {
            filters.push(Filter::parse(next));
        }
        workflows.insert(key, filters);
    }

    let parts = suffix
        .lines()
        .map(|line| Part::parse(line))
        .collect::<Vec<_>>();

    (workflows, parts)
}

pub fn part_one((workflows, parts): &(HashMap<String, Vec<Filter>>, Vec<Part>)) -> u32 {
    let mut sum = 0;

    for part in parts {
        let mut next: String = "in".to_string();

        loop {
            if next == "R" {
                break;
            }

            if next == "A" {
                sum += part.0 + part.1 + part.2 + part.3;
                break;
            }

            let mut filters = workflows[&next].iter();

            'inner: while let Some(filter) = filters.next() {
                if let Some(filtered_part) = filter.apply(part) {
                    next = filtered_part.to_string();
                    break 'inner;
                }
            }
        }
    }
    sum
}

pub fn part_two((workflows, _): &(HashMap<String, Vec<Filter>>, Vec<Part>)) -> u64 {
    let mut result = 0;
    let mut queue = vec![];

    // add ranges (1..=4000) for each char 'x', 'm', 'a', 's'
    queue.push(("in", 0, [(1, 4000); 4]));

    while let Some((key, index, part)) = queue.pop() {
        if key == "A" {
            result += part
                .iter()
                .map(|(start, end)| (end - start + 1) as u64)
                .product::<u64>(); // calculate the product of all the ranges
            continue;
        }
        if key == "R" {
            continue;
        }

        // follow the workflow if the range is contained,
        // if not, split into relevant sub-ranges.
        match workflows[key][index] {
            Filter::Send(next) => queue.push((next, 0, part)),
            Filter::Less(c_idx, v, next) => {
                let (start_idx, end_idx) = part[c_idx];

                if start_idx >= v {
                    queue.push((key, index + 1, part));
                } else if end_idx < v {
                    queue.push((next, 0, part));
                } else {
                    // open ended ranges, split in two and push both ranges
                    let mut first = part;
                    first[c_idx] = (start_idx, v - 1);
                    queue.push((next, 0, first));

                    let mut second = part;
                    second[c_idx] = (v, end_idx);
                    queue.push((key, index + 1, second));
                }
            }
            Filter::Greater(c_idx, v, next) => {
                let (start_idx, end_idx) = part[c_idx];

                if end_idx <= v {
                    queue.push((key, index + 1, part));
                } else if start_idx > v {
                    queue.push((next, 0, part));
                } else {
                    // open ended ranges, split in two and push both ranges
                    let mut first = part;
                    first[c_idx] = (start_idx, v);
                    queue.push((key, index + 1, first));

                    let mut second = part;
                    second[c_idx] = (v + 1, end_idx);
                    queue.push((next, 0, second));
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use crate::year2023::day19::{input, part_one, part_two, Filter};

    const EXAMPLE: &str = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn input_test() {
        let (workflows, parts) = input(EXAMPLE);

        assert_eq!(11, workflows.len());
        assert_eq!(5, parts.len());

        assert_eq!(
            vec![
                Filter::parse("a<2006:qkq"),
                Filter::parse("m>2090:A"),
                Filter::parse("rfg")
            ],
            workflows["px"]
        );
    }

    #[test]
    fn part_one_test() {
        assert_eq!(19114, part_one(&input(EXAMPLE)));
    }

    #[test]
    fn part_two_test() {
        assert_eq!(167409079868000, part_two(&input(EXAMPLE)));
    }

    #[test]
    fn filter_parse_test() {
        assert_eq!(Filter::Less(2, 2006, "qkq"), Filter::parse(r"a<2006:qkq"));
        assert_eq!(Filter::Greater(3, 3448, "A"), Filter::parse(r"s>3448:A"));
        assert_eq!(Filter::Send("lnx"), Filter::parse(r"lnx"));
    }
}
