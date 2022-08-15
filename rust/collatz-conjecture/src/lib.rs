pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        _ => iterate(n),
    }
}

fn iterate(n: u64) -> Option<u64> {
    let mut value = n;
    let mut steps = 0;
    while value > 1 {
        value = if value % 2 == 0 {
            value / 2
        } else {
            let mult = value.checked_mul(3);
            if mult.is_none() {
                return None;
            }

            let add = mult.unwrap().checked_add(1);
            if add.is_none() {
                return None;
            }

            add.unwrap()
        };
        steps += 1;
    }
    Some(steps)
}
