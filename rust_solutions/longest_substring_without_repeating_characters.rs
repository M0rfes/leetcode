use std::collections::HashMap;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_index = HashMap::new();
        let mut max_len = 0;
        let mut start = 0;
        for (i, c) in s.chars().enumerate() {
            if let Some(&prev_idx) = char_index.get(&c) {
                if prev_idx >= start {
                    start = prev_idx + 1;
                }
            }
            char_index.insert(c, i);
            max_len = max_len.max(i - start + 1);
        }
        max_len as i32
    }
}