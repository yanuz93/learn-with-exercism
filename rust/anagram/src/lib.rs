use std::collections::HashSet;

pub fn to_lower_and_sorted_to_lower(word: &str) -> (String, Vec<char>) {
    let lower: String = word.to_owned().to_lowercase();
    let mut chars: Vec<char> = lower.chars().collect();
    chars.sort_unstable();

    (lower, chars)
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let (lower, sorted) = to_lower_and_sorted_to_lower(word);

    possible_anagrams
        .iter()
        .copied()
        .filter(|&candidate| {
            let (lower_candidate, sorted_candidate) = to_lower_and_sorted_to_lower(candidate);
            lower != lower_candidate && sorted == sorted_candidate
        })
        .collect()
}
