use std::cmp::Ordering;

pub fn find<T, S>(array: S, key: T) -> Option<usize>
where
    T: Ord,
    S: AsRef<[T]>,
{
    let array = array.as_ref();

    if array.is_empty() {
        return None;
    }

    let middle_index = array.len() / 2;

    match &array[middle_index].cmp(&key) {
        Ordering::Less => find(&array[middle_index+1..], key)
            .map(|p| p + middle_index + 1),
        Ordering::Equal => Some(middle_index),
        Ordering::Greater => find(&array[0..middle_index], key),
    }
}
