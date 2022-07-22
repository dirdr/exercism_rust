use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let reference: HashMap<char, char> = HashMap::from([
        (']', '['),
        (')', '('),
        ('}', '{'),
    ]);
    for el in string.chars() {
        match el {
           '{' | '[' | '(' => stack.push(el),
            '}' | ']' | ')' => {
                let opening_ref = reference.get(&el);
                let top = stack.pop();
                if Some(opening_ref) != Some(top.as_ref()) {return false;}
            },
            _ => (),
        }
    }
    stack.is_empty()
}
