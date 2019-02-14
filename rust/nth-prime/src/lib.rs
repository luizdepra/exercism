fn is_prime(n: u32) -> bool {
    let limit = (n as f64).sqrt() as u32 + 1;
    !(3..limit).any(|u| n % u == 0)
}

pub fn nth(n: u32) -> u32 {
    match n {
        n if n == 0 => 2,
        n => (3..).step_by(2)
            .filter(|u| is_prime(*u))
            .nth((n - 1) as usize)
            .expect(r"¯\_(ツ)_/¯"),
    }
}
