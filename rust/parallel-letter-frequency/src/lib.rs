use std::collections::HashMap;
use std::sync::mpsc::channel;

use rayon;

fn count_letters(lines: &[&str]) -> HashMap<char, usize> {
    let mut result = HashMap::new();

    lines.iter().for_each(|l| {
        l.to_lowercase()
            .chars()
            .filter(|c| c.is_alphabetic())
            .for_each(|c| *result.entry(c).or_default() += 1);
    });

    result
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let chunk_size = input.len() / worker_count + 1;

    rayon::scope(|s| {
        let (sender, receiver) = channel();

        let spawned_workers_count = input
            .chunks(chunk_size)
            .map(|l| {
                let sender = sender.clone();
                s.spawn(move |_| {
                    let res = count_letters(l);
                    sender.send(res).unwrap();
                })
            })
            .count();

        let mut result = HashMap::new();
        for _ in 0..spawned_workers_count {
            let partial_result = receiver.recv().unwrap();
            partial_result
                .iter()
                .for_each(|(&k, &v)| *result.entry(k).or_default() += v);
        }

        result
    })
}
