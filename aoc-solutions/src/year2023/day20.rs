//! # Day 20: Pulse Propagation

use core::fmt;
use std::collections::{HashMap, VecDeque};

use utils::num::Integer;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Pulse {
    High,
    Low,
}

impl fmt::Display for Pulse {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    On,
    Off,
}

impl std::ops::Not for State {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::On => Self::Off,
            Self::Off => Self::On,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Module<'a> {
    Broadcaster(Vec<&'a str>),
    FlipFlop(State, Vec<&'a str>),
    Conjunction(HashMap<String, Pulse>, Vec<&'a str>),
}

impl<'a> Module<'a> {
    fn parse(line: &'a str) -> (String, Self) {
        let (prefix, suffix) = line.split_once(" -> ").unwrap();
        let next = suffix.split(',').map(|n| n.trim()).collect::<Vec<_>>();
        let id = prefix[1..].to_string();

        match prefix.chars().nth(0).unwrap() {
            '%' => (id, Module::FlipFlop(State::Off, next)),
            '&' => (id, Module::Conjunction(HashMap::new(), next)),
            _ => (prefix.to_string(), Module::Broadcaster(next)),
        }
    }

    fn propagate(&mut self, pulse: Pulse, from: &str) -> Vec<(&'a str, Pulse)> {
        match (self, pulse) {
            (Module::Broadcaster(next), _) => next.iter().map(|&n| (n, pulse)).collect::<Vec<_>>(),
            (Module::FlipFlop(_, _), Pulse::High) => vec![],
            (Module::FlipFlop(state, next), Pulse::Low) => {
                *state = !state.to_owned();

                match state {
                    State::On => next.iter().map(|&n| (n, Pulse::High)).collect::<Vec<_>>(),
                    State::Off => next.iter().map(|&n| (n, Pulse::Low)).collect::<Vec<_>>(),
                }
            }
            (Module::Conjunction(history, next), _) => {
                history.insert(from.to_string(), pulse);

                let next_pulse = history
                    .values()
                    .all(|&p| p == Pulse::High)
                    .then(|| Pulse::Low)
                    .unwrap_or_else(|| Pulse::High);

                next.iter().map(|&n| (n, next_pulse)).collect::<Vec<_>>()
            }
        }
    }
}

pub fn input(raw: &str) -> HashMap<String, Module> {
    let mut modules: HashMap<String, Module> = HashMap::new();
    let mut module_input: HashMap<String, Vec<String>> = HashMap::new();

    for line in raw.lines() {
        let (id, module) = Module::parse(line);

        match &module {
            Module::FlipFlop(_, next) | Module::Conjunction(_, next) => {
                next.iter().for_each(|&n| {
                    module_input
                        .entry(n.to_string())
                        .or_insert(vec![])
                        .push(id.to_string())
                });
            }
            _ => {}
        }
        modules.insert(id, module);
    }

    for (id, module) in modules.iter_mut() {
        if let Module::Conjunction(history, _) = module {
            module_input[id].iter().for_each(|input| {
                history.insert(input.to_string(), Pulse::Low);
            });
        }
    }

    modules
}

pub fn part_one(input: &HashMap<String, Module>) -> usize {
    find_pulses(&mut input.clone())
}

pub fn part_two(input: &HashMap<String, Module>) -> u128 {
    find_cycles(input)
}

pub fn find_pulses(lookup: &mut HashMap<String, Module>) -> usize {
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut button_presses = 0;

    loop {
        button_presses += 1;

        let mut queue: VecDeque<(&str, &str, Pulse)> = VecDeque::new();
        queue.push_back(("button", "broadcaster", Pulse::Low));

        while let Some((from, to, pulse)) = queue.pop_front() {
            match pulse {
                Pulse::High => high_pulses += 1,
                Pulse::Low => low_pulses += 1,
            }
            // println!("{} -{}-> {}", from, pulse, to);

            if let Some(module) = lookup.get_mut(to) {
                module
                    .propagate(pulse, from)
                    .iter()
                    .for_each(|&(next, next_pulse)| {
                        queue.push_back((to, next, next_pulse));
                    })
            }
        }

        let on_count = lookup
            .values()
            .filter_map(|m| matches!(m, Module::FlipFlop(State::On, _)).then(|| 1))
            .sum::<usize>();

        if on_count == 0 || button_presses >= 1000 {
            break;
        }
    }

    // calculate the total number of presses
    low_pulses * high_pulses * (1000 / button_presses as usize).pow(2)
}

fn parents(modules: &HashMap<String, Module>, child: &str) -> Vec<String> {
    let mut parents: Vec<String> = Vec::new();

    for (id, module) in modules {
        let next = match &module {
            Module::Broadcaster(next) => next,
            Module::FlipFlop(_, next) => next,
            Module::Conjunction(_, next) => next,
        };

        if next.iter().any(|&n| n == child) {
            parents.push(id.to_string());
        }
    }

    parents
}

pub fn find_cycles(lookup: &HashMap<String, Module>) -> u128 {
    let mut cycles = Vec::new();

    // first find the parent of the 'rx' module
    let parent = parents(lookup, "rx");

    // then find all if its parents. These are the ones we want to align
    let grandparent = parents(lookup, parent.first().unwrap());

    for module in grandparent {
        let next_cycle = find_module_cycle(&mut lookup.clone(), module);
        cycles.push(next_cycle);
    }

    cycles.iter().fold(1, |acc, &next| acc.lcm(&next))
}

pub fn find_module_cycle(lookup: &mut HashMap<String, Module>, module: String) -> u128 {
    let mut button_presses = 0;

    loop {
        button_presses += 1;

        let mut queue: VecDeque<(&str, &str, Pulse)> = VecDeque::new();
        queue.push_back(("button", "broadcaster", Pulse::Low));

        while let Some((from, to, pulse)) = queue.pop_front() {
            if to == module && pulse == Pulse::Low {
                return button_presses;
            }

            if let Some(module) = lookup.get_mut(to) {
                module
                    .propagate(pulse, from)
                    .iter()
                    .for_each(|&(next, next_pulse)| {
                        queue.push_back((to, next, next_pulse));
                    })
            }
        }

        if button_presses >= 10_000 {
            panic!("No cycles detected")
        }
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, vec};

    use crate::year2023::day20::{input, part_one, part_two, Module, Pulse, State};

    const EXAMPLE: &str = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const EXAMPLE_TWO: &str = r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn input_test() {
        let input = input(EXAMPLE);

        assert!(
            matches!(&input["broadcaster"], Module::Broadcaster(next) if *next == vec!["a", "b", "c"])
        );
        assert!(matches!(&input["a"], Module::FlipFlop(State::Off, next) if *next == vec!["b"]));
        assert!(matches!(&input["b"], Module::FlipFlop(State::Off, next) if *next == vec!["c"]));
        assert!(matches!(&input["c"], Module::FlipFlop(State::Off, next) if *next == vec!["inv"]));

        if let Module::Conjunction(history, next) = &input["inv"] {
            assert_eq!(Pulse::Low, history["c"]);
            assert_eq!(vec!["a"], *next)
        } else {
            panic!("No entry for 'inv'")
        }
    }

    #[test]
    fn part_one_test() {
        assert_eq!(32000000, part_one(&input(EXAMPLE)));
        assert_eq!(11687500, part_one(&input(EXAMPLE_TWO)));
    }

    #[test]
    fn part_two_test() {
        // input does not have an 'rx' tag and cannot be validated here
        assert_eq!(32000000, part_two(&input(EXAMPLE)));
    }

    const NO_PULSE: Vec<(&str, Pulse)> = vec![];
    const A: &str = "a";
    const B: &str = "b";

    #[test]
    fn flip_flop_test() {
        let mut flip_flop = Module::FlipFlop(State::On, vec![A, B]);

        assert_eq!(NO_PULSE, flip_flop.propagate(Pulse::High, "c"));
        assert!(matches!(flip_flop, Module::FlipFlop(State::On, _)));
        assert_eq!(
            vec![(A, Pulse::Low), (B, Pulse::Low)],
            flip_flop.propagate(Pulse::Low, "c")
        );
        assert!(matches!(flip_flop, Module::FlipFlop(State::Off, _)));
        assert_eq!(
            vec![(A, Pulse::High), (B, Pulse::High)],
            flip_flop.propagate(Pulse::Low, "c")
        );
        assert!(matches!(flip_flop, Module::FlipFlop(State::On, _)));
    }

    #[test]
    fn conjunction_test() {
        let input_map = HashMap::from([
            (String::from("c"), Pulse::Low),
            (String::from("d"), Pulse::Low),
        ]);

        let mut conjuction = Module::Conjunction(input_map, vec![A, B]);

        assert_eq!(
            vec![(A, Pulse::Low), (B, Pulse::Low)],
            conjuction.propagate(Pulse::High, "c")
        );
        assert_eq!(
            vec![(A, Pulse::Low), (B, Pulse::Low)],
            conjuction.propagate(Pulse::Low, "d")
        );
        assert_eq!(
            vec![(A, Pulse::High), (B, Pulse::High)],
            conjuction.propagate(Pulse::High, "d")
        );
    }
}
