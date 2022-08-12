fn extract(number: u64, factor: u64, factors: &mut Vec<u64>) -> u64 {
    let mut value = number;

    while value % factor == 0 {
        value /= factor;
        factors.push(factor);
    }

    value
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut value = n;
    let mut factors = vec![];

    value = extract(value, 2, &mut factors);

    let mut odds = (3..).step_by(2);
    while value > 1 {
        let factor = odds.next().unwrap();
        value = extract(value, factor, &mut factors);
    }

    factors
}
