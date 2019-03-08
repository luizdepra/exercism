use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSlice;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Sync>(first_list: &[T], second_list: &[T]) -> Comparison {
    let first_length: usize = first_list.len();
    let second_length: usize = second_list.len();
    let is_first_empty: bool = first_list.is_empty();
    let is_second_empty: bool = second_list.is_empty();

    let sublist_or_superlist = || -> Comparison {
        if first_list == second_list {
            Comparison::Equal
        } else if is_first_empty || second_list.par_windows(first_length).any(|s| s == first_list) {
            Comparison::Sublist
        } else if is_second_empty || first_list.par_windows(second_length).any(|s| s == second_list) {
            Comparison::Superlist
        } else {
            Comparison::Unequal
        }
    };

    sublist_or_superlist()
}
