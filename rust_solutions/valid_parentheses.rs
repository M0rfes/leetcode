fn is_valid(s: String) -> bool {
    if s.chars().count() % 2 != 0 {
        return false;
    }
    let map = HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);
    let mut stack = vec![];
    for c in s.chars() {
        if let Some(cc) = map.get(&c) {
            stack.push(cc)
        } else {
            let Some(top) = stack.pop() else {
                return false;
            };
            if *top != c {
                return false;
            }
        }
    }
    return stack.is_empty();
}
