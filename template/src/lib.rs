use wasmcloud_advent_of_code_interface::{AdventOfCode, AdventOfCodeReceiver};
use wasmbus_rpc::actor::prelude::*;

// TODO: Fill input text files with the input from Advent of Code
const INPUT_PART_1: &str = include_str!("../input_part_1.txt");
const INPUT_PART_2: &str = include_str!("../input_part_2.txt");

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, AdventOfCode)]
struct Day1Actor {}

#[async_trait]
impl AdventOfCode for Day1Actor {
    async fn part_one(&self, _ctx: &Context) -> RpcResult<String> {
        Ok(format!("Solution: {}", part_1()))
    }
    async fn part_two(&self, _ctx: &Context) -> RpcResult<String> {
        Ok(format!("Solution: {}", part_2()))
    }
}

/// TODO: Implement the solution for part 1
fn part_1() -> u32 {
    0
}

/// TODO: Implement the solution for part 2
fn part_2() -> u32 {
    0
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    #[test]
    /// Helper function that you can use to test, make sure to run `cargo test --target <your_native_target>`
    fn part_1_test() {
        println!("{:?}", part_2());
        assert!(part_1() > 0);
    }

    #[test]
    /// Helper function that you can use to test, make sure to run `cargo test --target <your_native_target>`
    fn part_2_test() {
        println!("{:?}", part_2());
        assert!(part_2() > 0);
    }
}
