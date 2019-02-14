pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::with_capacity(string.len() / 2);

    for c in string.chars() {
        if c == '[' || c == '{' || c == '(' {
            stack.push(c);
        } else if c == ']' || c == '}' || c == ')' {
            if let Some(o) = stack.pop() {
                if (c == ']' && o != '[') ||
                   (c == '}' && o != '{') ||
                   (c == ')' && o != '(') {
                       return false;
                   }
            } else {
                return false;
            }
        }
    }

    stack.is_empty()
}
