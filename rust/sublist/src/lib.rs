#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let is_sublist = |a: &[T], b: &[T]| b.is_empty() || a.windows(b.len()).any(|s| s == b);

    if first_list == second_list {
        Comparison::Equal
    } else if is_sublist(second_list, first_list) {
        Comparison::Sublist
    } else if is_sublist(first_list, second_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
