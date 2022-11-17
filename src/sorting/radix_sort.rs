/// T(n) = O(n * b)
/// S(n) = O(n + b)
/// `n`: arr length
/// `b`: the base (radix)
pub fn radix_sort(arr: &mut [usize]) {
    let arr_len = arr.len();

    if arr_len <= 1 {
        return;
    }

    let max_num = match arr.iter().max() {
        Some(&x) => x,
        None => return,
    };

    // make radix a power of 2 close to arr.len() for optimal runtime
    let radix = arr_len.next_power_of_two();
    // counting sort by each digit from least to most significant
    let mut place = 1;
    while place <= max_num {
        // count digit occurrences
        let digit_of = |x: usize| x / place % radix;
        let mut counter = vec![0; radix];
        for &x in arr.iter() {
            counter[digit_of(x)] += 1;
        }

        // compute last index of each digit
        for i in 1..radix {
            counter[i] += counter[i - 1];
        }

        // write elements to their new indices
        for &x in arr.to_owned().iter().rev() {
            counter[digit_of(x)] -= 1;
            arr[counter[digit_of(x)]] = x;
        }

        place *= radix;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_sort() {
        // unsorted
        let mut arr = vec![5, 7, 2, 3, 1];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 7]);

        // descending
        let mut arr = vec![5, 4, 3, 2, 1];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // accending
        let mut arr = vec![1, 2, 3, 4, 5];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // one element
        let mut arr: Vec<usize> = vec![2];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![2]);

        // empty
        let mut arr: Vec<usize> = vec![];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }
}
