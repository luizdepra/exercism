const SURE: &'static str = "Sure.";
const WHOA: &'static str = "Whoa, chill out!";
const CALM_DOWN: &'static str = "Calm down, I know what I'm doing!";
const FINE: &'static str = "Fine. Be that way!";
const WHATEVER: &'static str = "Whatever.";

pub fn reply(message: &str) -> &str {
    let is_yelling =
        |s: &str| s.chars().any(|c| c.is_alphabetic()) && !s.chars().any(|c| c.is_lowercase());
    let is_questioning = |s: &str| s.ends_with("?");

    let message = message.trim();
    match message {
        message if message.is_empty() => FINE,
        message if is_yelling(message) && is_questioning(message) => CALM_DOWN,
        message if is_yelling(message) => WHOA,
        message if is_questioning(message) => SURE,
        _ => WHATEVER,
    }
}
