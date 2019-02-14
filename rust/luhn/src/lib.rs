/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let clean_code = code.replace(" ", "");
    if clean_code.len() < 2 {
        return false;
    }

    let length = clean_code.len();
    clean_code.chars()
        .map(|c| c.to_digit(10))
        .enumerate()
        .try_fold(0, move |sum, (i, o)| o.map(move |mut d| {
            if (length - i) % 2 == 0 {
                d *= 2;
                if d > 9 {
                    d -= 9;
                }
            }
            sum + d
        }))
        .map_or(false, |x| x % 10 == 0)
}
