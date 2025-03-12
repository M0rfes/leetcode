fn roman_to_int(s: String) -> i32 {
    let map: HashMap<&str, i32> = HashMap::from([
        ("I", 1),
        ("IV", 4),
        ("V", 5),
        ("IX", 9),
        ("X", 10),
        ("XL", 40),
        ("L", 50),
        ("XC", 90),
        ("C", 100),
        ("CD", 400),
        ("D", 500),
        ("CM", 900),
        ("M", 1000),
    ]);

    let mut i = 0;
    let mut count = 0;

    while i < s.len() {
        if i + 1 < s.len() {
            let two_char = &s[i..i + 2];
            if let Some(&value) = map.get(two_char) {
                count += value;
                i += 2;
                continue;
            }
        }
        let one_char = &s[i..i + 1];
        if let Some(&value) = map.get(one_char) {
            count += value;
            i += 1;
        } else {
            panic!("Unknown symbol: {}", one_char);
        }
    }
    count
}
