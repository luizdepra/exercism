pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|n| factors.iter().filter(|f| **f != 0).any(|f| n % f == 0))
        .sum()
}
