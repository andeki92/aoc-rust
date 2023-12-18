//! # Day 17: Clumsy Crucible

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use utils::{
    grid::Grid,
    point::{Point, DOWN, RIGHT},
};

type SolutionType = i64;

pub fn input(raw: &str) -> Grid<i64> {
    Grid::parse(raw)
}

pub fn part_one(grid: &Grid<i64>) -> SolutionType {
    heat_loss::<1, 3>(grid).unwrap()
}

pub fn part_two(grid: &Grid<i64>) -> SolutionType {
    heat_loss::<4, 10>(grid).unwrap()
}

#[derive(Debug, PartialEq, Eq)]
pub struct State {
    cost: i64,
    node: Point,
    dir: Point,   // direction you entered the node
    steps: usize, // number of steps taken in the same direction
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn neighbours<const MIN: usize, const MAX: usize>(&self, grid: &Grid<i64>) -> Vec<Self> {
        let mut neighbours = vec![];

        for node in self.node.cardinal() {
            if self.node + self.dir != node && self.steps < MIN {
                // we need to move a MIN numver of steps before we can turn
                continue;
            }

            if self.node + self.dir == node && self.steps == MAX {
                // if we've already moved MAX steps in this direction, we cannot continue
                continue;
            }

            if self.node - self.dir == node {
                // we came from this direction, no need to go back that way
                continue;
            }

            if grid.contains(node) {
                let dir = node - self.node;
                let cost = self.cost + grid[node];
                let steps = if self.node + self.dir == node {
                    self.steps + 1
                } else {
                    1
                };

                neighbours.push(State {
                    node,
                    cost,
                    dir,
                    steps,
                })
            }
        }
        neighbours
    }
}

fn heat_loss<const MIN: usize, const MAX: usize>(grid: &Grid<i64>) -> Option<i64> {
    let start = Point::new(0, 0);
    let end = Point::new(grid.width - 1, grid.height - 1);

    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();

    let right_cost = (1..=MIN).map(|x_off| grid[start + (RIGHT * x_off)]).sum();
    let down_cost = (1..=MIN).map(|y_off| grid[start + (DOWN * y_off)]).sum();

    // move right
    queue.push(State {
        node: start + (RIGHT * MIN),
        cost: right_cost,
        dir: RIGHT,
        steps: MIN,
    });

    // move down
    queue.push(State {
        node: start + (DOWN * MIN),
        cost: down_cost,
        dir: DOWN,
        steps: MIN,
    });

    while let Some(state) = queue.pop() {
        if state.node == end && state.steps >= MIN {
            return Some(state.cost);
        }

        for neighbour in state.neighbours::<MIN, MAX>(grid) {
            // since the queue is ordered, if we encounter the node a second
            // time the weight of this will be lower. We can therefore check
            // if the insert succeeds and if not skip this neighbour
            if seen.insert((neighbour.node, neighbour.dir, neighbour.steps)) {
                queue.push(neighbour)
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use utils::point::Point;

    use crate::year2023::day17::{input, part_one, part_two};

    const EXAMPLE: &str = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn input_test() {
        let grid = input(EXAMPLE);
        assert_eq!(13, grid.height);
        assert_eq!(13, grid.width);
        assert_eq!(13 * 13, grid.data.len());
        assert_eq!(2, grid[Point::new(0, 0)]);
        assert_eq!(4, grid[Point::new(2, 3)]);
        assert_eq!(3, grid[Point::new(12, 12)]);
    }

    #[test]
    fn part_one_test() {
        assert_eq!(102, part_one(&input(EXAMPLE)));
    }

    #[test]
    fn part_two_test() {
        assert_eq!(94, part_two(&input(EXAMPLE)));
    }
}
