type InputType = String;
type SolutionType = u32;

pub fn input(_: &str) -> InputType {
    todo!()
}

pub fn part_one(_: &InputType) -> SolutionType {
    todo!()
}

pub fn part_two(_: &InputType) -> SolutionType {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::year2023::day1::{input, part_one, part_two};

    const EXAMPLE: &str = r"";

    #[test]
    fn input_test() {
        assert_eq!("", input(EXAMPLE));
    }

    #[test]
    fn part_one_test() {
        assert_eq!(0, part_one(&input(EXAMPLE)));
    }

    #[test]
    fn part_two_test() {
        assert_eq!(0, part_two(&input(EXAMPLE)));
    }
}
