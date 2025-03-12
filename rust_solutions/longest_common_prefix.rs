fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut prefix = strs[0].clone(); // Start with the first string as the prefix

    for s in strs.iter().skip(1) { // Compare with all other strings
        while !s.starts_with(&prefix) {
            prefix.pop(); // Shorten the prefix
            if prefix.is_empty() {
                return String::new(); // No common prefix
            }
        }
    }

    prefix
}