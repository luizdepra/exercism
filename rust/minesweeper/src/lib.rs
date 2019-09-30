use std::char;

use itertools::iproduct;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield.iter().enumerate().map(|(y, row)| {
        row.char_indices().map(|(x, cell)| {
            match cell {
                ' ' => {
                    let count = count_neighbor_mines(minefield, x, y) as u32;
                    if count == 0 {
                        ' '
                    } else {
                        char::from_digit(count, 10).unwrap_or(' ')
                    }
                },
                c => c,
            }
        }).collect()
    }).collect()
}

fn count_neighbor_mines(minefield: &[&str], x: usize, y: usize) -> usize {
    let x = x as isize;
    let y = y as isize;

    iproduct!(y-1..=y+1, x-1..=x+1).filter(|(j, i)| {
        if *j < 0 || *i < 0 {
            return false;
        }

        if let Some(row) = minefield.get(*j as usize) {
            if row.chars().nth(*i as usize) == Some('*') {
                return true;
            }
        }

        false
    }).count()
}
