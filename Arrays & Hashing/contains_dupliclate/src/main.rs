use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut vistos: HashSet<i32> = HashSet::with_capacity(nums.len()); //Es mas rapido con with_capacity() que new()

    for num in nums.iter() {
        if !vistos.insert(*num) {
            return true;
        }
    }
    return false;
}

fn main() {
    let nums = vec![1, 2, 3, 4, 1];
    let result = contains_duplicate(nums);
    println!("Contains duplicate: {}", result);
}
