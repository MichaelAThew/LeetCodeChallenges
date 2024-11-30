pub fn is_anagram(s: String, t: String) -> bool {
    let mut s_chars: Vec<char> = s.chars().collect();
    let mut t_chars: Vec<char> = t.chars().collect();
    s_chars.sort_unstable();
    t_chars.sort_unstable();
    s_chars == t_chars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
        assert!(!is_anagram("rat".to_string(), "car".to_string()));
    }
}
