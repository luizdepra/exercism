use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mag_words = magazine.iter().fold(HashMap::new(), |mut map, x| {
        *map.entry(x).or_insert(0) += 1;
        map
    });

    let note_words = note.iter().fold(HashMap::new(), |mut map, x| {
        *map.entry(x).or_insert(0) += 1;
        map
    });

    note_words
        .iter()
        .all(|(k, v)| mag_words.get(*k).unwrap_or(&0) >= v)
}
