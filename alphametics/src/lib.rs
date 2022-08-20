use std::{collections::HashMap};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut left = input.split("+");
    for element in left {
        print!("{}", element);
    }
    unimplemented!("Solve the alphametic {:?}", input)
}
