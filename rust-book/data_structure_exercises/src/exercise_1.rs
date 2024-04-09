use std::collections::HashMap;

pub fn median(vec: &mut Vec<i32>) -> i32 {
    vec.sort();
    return vec[vec.len() / 2];
}

pub fn mode(vec: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for i in vec {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    let mut ret = 0;
    for (key, value) in map {
        if value > max {
            max = value;
            ret = *key;
        }
    }

    ret
}
