pub fn verse(n: i32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\n\
              Go to the store and buy some more, 99 bottles of beer on the wall.\n"
            .to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\n\
              Take it down and pass it around, no more bottles of beer on the wall.\n"
            .to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\n\
              Take one down and pass it around, 1 bottle of beer on the wall.\n"
            .to_string(),
        n => format!(
            "{0} bottles of beer on the wall, {0} bottles of beer.\n\
             Take one down and pass it around, {1} bottles of beer on the wall.\n",
            n,
            n - 1
        ),
    }
}

pub fn sing(start: i32, end: i32) -> String {
    (start..=end)
        .map(verse)
        .rev()
        .collect::<Vec<_>>()
        .join("\n")
}
