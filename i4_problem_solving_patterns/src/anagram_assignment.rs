use std::collections::HashMap;

pub fn run() {
    let _result = anagram("alexander", "aerlexnad");

    println!("{}", _result);
}

fn anagram(name_one: &str, name_two: &str) -> bool {
    if name_one.len() != name_two.len() {
        return false;
    }

    let mut lookup = HashMap::<u8, u8>::new();

    for idx in name_one.bytes() {
        let look = match lookup.get(&idx) {
            Some(item) => *item,
            None => 0,
        };

        if lookup.contains_key(&idx) {
            lookup.insert(idx, look + 1);
        } else {
            lookup.insert(idx, 1);
        }
    }

    for idx in name_two.bytes() {
        let look = match lookup.get(&idx) {
            Some(item) => *item,
            None => 0,
        };

        if look != 0 {
            lookup.insert(idx, look - 1);
        } else {
            return false;
        }
    }

    true
}
