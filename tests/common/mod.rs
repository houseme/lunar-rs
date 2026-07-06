pub fn norm(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
