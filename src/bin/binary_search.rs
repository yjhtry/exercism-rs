use std::cmp::Ordering;

pub fn find<R: AsRef<[T]>, T: Ord>(array: R, key: T) -> Option<usize> {
    let array = array.as_ref();
    let middle = array.len() / 2;

    match key.cmp(array.get(middle)?) {
        Ordering::Equal => Some(middle),
        Ordering::Less => find(&array[..middle], key),
        Ordering::Greater => find(&array[(middle + 1)..], key).map(|i| i + middle + 1),
    }
}

fn main() {
    let arr = [1, 2, 3, 4];

    println!("{:?}", &arr[0..0]);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn finds_a_value_in_an_array_with_one_element() {
        assert_eq!(find([6], 6), Some(0));
    }
    #[test]
    fn finds_a_value_in_the_middle_of_an_array() {
        assert_eq!(find([1, 3, 4, 6, 8, 9, 11], 6), Some(3));
    }
    #[test]
    fn finds_a_value_at_the_beginning_of_an_array() {
        assert_eq!(find([1, 3, 4, 6, 8, 9, 11], 1), Some(0));
    }
    #[test]
    fn finds_a_value_at_the_end_of_an_array() {
        assert_eq!(find([1, 3, 4, 6, 8, 9, 11], 11), Some(6));
    }
    #[test]
    fn finds_a_value_in_an_array_of_odd_length() {
        assert_eq!(
            find([1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 634], 144),
            Some(9)
        );
    }
    #[test]
    fn finds_a_value_in_an_array_of_even_length() {
        let value = find([1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377], 21);
        assert_eq!(value, Some(5));
    }
    #[test]
    fn identifies_that_a_value_is_not_included_in_the_array() {
        assert_eq!(find([1, 3, 4, 6, 8, 9, 11], 7), None);
    }
    #[test]
    fn a_value_smaller_than_the_array_s_smallest_value_is_not_found() {
        assert_eq!(find([1, 3, 4, 6, 8, 9, 11], 0), None);
    }
    #[test]
    fn a_value_larger_than_the_array_s_largest_value_is_not_found() {
        assert_eq!(find([1, 3, 4, 6, 8, 9, 11], 13), None);
    }
    #[test]
    fn nothing_is_found_in_an_empty_array() {
        assert_eq!(find([], 1), None);
    }
    #[test]
    fn nothing_is_found_when_the_left_and_right_bounds_cross() {
        assert_eq!(find([1, 2], 0), None);
    }
    #[test]
    #[cfg(feature = "generic")]
    fn works_for_arrays() {
        assert_eq!(find([6], 6), Some(0));
    }
    #[test]
    #[cfg(feature = "generic")]
    fn works_for_vec() {
        let vector = vec![6];
        assert_eq!(find(vector, 6), Some(0));
        assert_eq!(find(vector, 6), Some(0));
    }
    #[test]
    #[cfg(feature = "generic")]
    fn works_for_str_elements() {
        assert_eq!(find(["a"], "a"), Some(0));
        assert_eq!(find(["a", "b"], "b"), Some(1));
    }
}
