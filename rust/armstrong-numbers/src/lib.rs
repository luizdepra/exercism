pub fn is_armstrong_number(num: u32) -> bool {
    let mut value = num;
    let mut digits = vec![];
    while value > 0 {
        digits.push(value % 10);
        value /= 10;
    }

    let sum: u32 = digits.iter().map(|x| x.pow(digits.len() as u32)).sum();
    sum == num
}
