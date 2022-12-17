fn partition<T: Ord>(arr: &mut [T], left: isize, right: isize) -> isize {
    let pivot = right;

    let mut i = left - 1;

    for j in left..=right - 1 {
        if arr[j as usize] <= arr[pivot as usize] {
            i += 1;
            arr.swap(i as usize, j as usize)
        }
    }

    arr.swap((i + 1) as usize, pivot as usize);
    i + 1
}

fn quicksort<T: Ord>(arr: &mut [T]) {
    _quicksort(arr, 0, (arr.len() - 1) as isize);
}

fn _quicksort<T: Ord>(arr: &mut [T], left: isize, right: isize) {
    if left <= right {
        let pivot_index = partition(arr, left, right);
        _quicksort(arr, left, pivot_index - 1);
        _quicksort(arr, pivot_index + 1, right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partitioning_methods_exists_and_works() {
        let mut arr = vec![11, 9, 6, 5, 14, 21, 10];
        let len = arr.len() - 1;
        let expected_partition_index = 3;

        let partition_index = partition(&mut arr, 0, len as isize);

        assert_eq!(partition_index, expected_partition_index);
        assert_eq!(arr, [9, 6, 5, 10, 14, 21, 11]);
    }

    #[test]
    fn quick_sort_sorts_an_arr() {
        let mut arr = vec![11, 9, 6, 5, 14, 21, 10];

        quicksort(&mut arr);

        assert_eq!(arr, [5, 6, 9, 10, 11, 14, 21]);
    }
}
