fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut s_chars: Vec<char> = s.chars().collect();
    let mut t_chars: Vec<char> = t.chars().collect();
    s_chars.sort();
    t_chars.sort();
    return s_chars == t_chars;
}

fn main() {
    let s1 = String::from("anagram");
    let t1 = String::from("nagaram");
    let result1 = is_anagram(s1.clone(), t1.clone());
    println!("Is '{}' an anagram of '{}'? {}", s1, t1, result1);

    let s2 = String::from("rat");
    let t2 = String::from("car");
    let result2 = is_anagram(s2.clone(), t2.clone());
    println!("Is '{}' an anagram of '{}'? {}", s2, t2, result2);
}
