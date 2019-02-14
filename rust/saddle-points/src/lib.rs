pub fn greater_than_line(x: usize, y: usize, input: &[Vec<u64>]) -> bool {
    let value = input[x][y];

    for i in 0..input[0].len() {
        if input[x][i] > value {
            return false;
        }
    }

    true
}

pub fn less_than_column(x: usize, y: usize, input: &[Vec<u64>]) -> bool {
    let value = input[x][y];

    for i in 0..input.len() {
        if input[i][y] < value {
            return false;
        }
    }

    true
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    for (x, line) in input.iter().enumerate() {
        for (y, _) in line.iter().enumerate() {
            if greater_than_line(x, y, input) && less_than_column(x, y, input) {
                result.push((x, y));
            }
        }
    }

    result
}
