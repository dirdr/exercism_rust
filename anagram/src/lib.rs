use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {

    let mut answer: HashSet<&str> = HashSet::new();
    let mut model: HashMap<char, i32> = HashMap::new();

    for letter in word.to_lowercase().chars() {
        model.entry(letter).and_modify(|e| {*e += 1}).or_insert(1);
    }
    
    for &candidate in possible_anagrams {

        let mut dont_add: bool = false;
        let mut map = model.clone();

        for letter in candidate.to_lowercase().chars() {

            let entry = map.entry(letter);
            let mutb : &mut bool = &mut dont_add;

            match entry {
                Entry::Vacant(_v) => {
                    *mutb = true;
                    break;
                },
                Entry::Occupied(mut o) => {
                    let value = *o.get();
                    if value == 0 {
                        *mutb = true;
                        break;
                    } else {
                        o.insert(value - 1);
                    }
                },
            }
        }
        let mut count:i32 = 0;
        for (_key, value) in map.iter() {
            count += value;
        }
        if count == 0 && dont_add == false {
            answer.insert(candidate);
        }
    } 
    answer.iter().filter(|e| {
        let elow = e.to_lowercase();
        elow != word.to_lowercase()
    }).cloned().collect()
}
