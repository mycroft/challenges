use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match _first_list.len().cmp(&_second_list.len()) {
        Ordering::Equal => {
            if _first_list == _second_list { Comparison::Equal } else { Comparison::Unequal }
        },
        Ordering::Greater => {
            if contains(_second_list, _first_list) { 
                Comparison::Superlist
            } else {
                Comparison::Unequal 
            }
        },
        Ordering::Less => {
            if contains(_first_list, _second_list) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
    }
}

pub fn contains<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    a.len() == 0 || b.windows(a.len()).any(|c| c == a)
}