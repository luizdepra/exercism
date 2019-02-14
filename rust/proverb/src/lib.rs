pub fn build_proverb(list: &[&str]) -> String {
    let mut verses = list
        .windows(2)
        .map(|s| format!("For want of a {0} the {1} was lost.", s[0], s[1]))
        .collect::<Vec<_>>();

    if let Some(s) = list.first() {
        verses.push(format!("And all for the want of a {0}.", s));
    };

    verses.join("\n")
}
