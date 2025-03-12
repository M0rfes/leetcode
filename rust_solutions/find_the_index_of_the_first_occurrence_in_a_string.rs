pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.is_empty() || needle.len() > haystack.len() {
        return -1;
    }

    let haystack = haystack.chars().collect::<Vec<char>>();
    let needle = needle.chars().collect::<Vec<char>>();
    let m = needle.len();
    let n = haystack.len();

    let mut window_start = 0usize;
    while (window_start <= n - m) {
        for i in 0..n {
            if needle[i] != haystack[window_start + i] {
                break;
            }

            if i == m - 1 {
                return window_start as i32;
            }
        }
        window_start += 1;
    }
    -1
}
