pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut new_minefield: Vec<String> = Vec::with_capacity(minefield.len());
    for (y, row) in minefield.iter().enumerate() {
        let mut new_row:String = String::with_capacity(row.len());
        for (x, cell) in row.chars().enumerate() {
            let new_cell: char = match cell {
                ' ' => {
                    let count: u8 = count_neighbor_mines(minefield, x, y) as u8;
                    if count == 0 {
                        ' '
                    } else {
                        (count + b'0') as char
                    }
                },
                c => c,
            };
            new_row.push(new_cell);
        }
        new_minefield.push(new_row);
    }

    new_minefield
}

fn count_neighbor_mines(minefield: &[&str], x: usize, y: usize) -> usize {
    let lower_y: usize = if y == 0 { y } else { y - 1 };
    let higher_y: usize = if y == minefield.len() - 1 { y } else { y + 1 };
    let lower_x: usize = if x == 0 { x } else { x - 1 };
    let higher_x: usize = if x == minefield[0].len() - 1 { x } else { x + 1 };

    let mut count: usize = 0;
    for ty in lower_y..=higher_y {
        for tx in lower_x..=higher_x {
            if minefield[ty].as_bytes()[tx] == b'*' {
                count += 1;
            }
        }
    }

    count
}
