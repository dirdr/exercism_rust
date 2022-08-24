use std::{collections::{HashMap, HashSet}, char::from_digit, u8, iter::{self}};
use itertools::Itertools;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let breaked = break_input(input);
    let brute_force = brute_force(&breaked);
    for map in brute_force {
        let valid = valid(&breaked, &map);
        if valid == true {
            return Some(map);
        }
    }
    None
}

pub fn brute_force(breaked_input: &Vec<&str>) -> Vec<HashMap<char, u8>> {
    let mut set = HashSet::new();

    for word in breaked_input {
       for letter in word.chars() {
            set.insert(letter);
        }
    }

    let combinations = (0..10).permutations(set.len());
    let vectorize: Vec<char> = set.into_iter().collect();
    let input_iter = iter::repeat(vectorize).take(combinations.clone().count());
    let zipped = input_iter.zip(combinations);

    let mut answer = Vec::new();

    zipped.for_each(|(letters, values)| {
        let mut temp = HashMap::new();
        for i in 0..letters.len() {
           temp.insert(letters[i], values[i]); 
        }
        answer.push(temp);
    });
    answer
}

pub fn valid(breaked_input: &Vec<&str>, solution: &HashMap<char, u8>) -> bool {
    let mut need_to_return = false;
    let mapped: Vec<u64> = breaked_input.iter().map(|word| {
        let string_rep = word.chars().map(|letter| {
            from_digit(*solution.get(&letter).unwrap() as u32, 10).unwrap()
        }).collect::<String>();
        if string_rep.chars().nth(0).unwrap() == '0' {
            need_to_return = true;
        }
        string_rep.parse::<u64>().unwrap()
    }).collect();

    if need_to_return {return false;}
    let mut left = 0;

    for i in 0..mapped.len()-1 {
        left += mapped[i];
    }
    left == mapped[mapped.len()-1]
}

pub fn break_input(input: &str) -> Vec<&str> {
    let mut container = Vec::new();
    let mut right_iter= input.split("==");
    let left_sent = right_iter.next().unwrap();

    for word in left_sent.split("+") {
        container.push(word.trim());
    }
    container.push(right_iter.next().unwrap().trim());
    container
}
