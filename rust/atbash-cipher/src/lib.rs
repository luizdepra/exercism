const TRANSLATION_VALUE:u8 = 'z' as u8 + 'a' as u8;

fn translate(ch: char) -> char {
    if !ch.is_alphabetic() {
        return ch;
    }

    (TRANSLATION_VALUE - ch as u8) as char
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain.to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(translate)
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|s| s.iter().cloned().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars()
        .filter(|c| !c.is_whitespace())
        .map(translate)
        .collect()
}
