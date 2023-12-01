use crate::{error::Result, Solution};

pub fn run(input: &str) -> Result<Solution> {
    todo!();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[ignore]
    #[test]
    fn matches_examples() {
        let example = r"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let solution = run(example).unwrap();

        assert_eq!(solution.part_one, "142");

        assert_eq!(solution.part_two, "");
    }
}
