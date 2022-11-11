/// 数组求和 迭代版
/// T(n) = O(n)
pub fn array_sum_v1(arr: &[i32]) -> i32 {
    // arr.iter().map(|v| *v as i64).sum()
    arr.iter().sum()
}

/// 数组求和 线性递归版
/// T(n) = O(n)
pub fn array_sum_v2(arr: &[i32], n: usize) -> i32 {
    if n < 1 {
        0
    } else {
        array_sum_v2(arr, n - 1) + arr[n - 1]
    }
}

/// 数组求和 二分递归版
/// T(n) = O(high - low + 1)
pub fn array_sum_v3(arr: &[i32], low: usize, high: usize) -> i32 {
    if low == high {
        arr[low]
    } else {
        let mid = (low + high) >> 1;
        array_sum_v3(arr, low, mid) + array_sum_v3(arr, mid + 1, high)
    }
}

#[cfg(test)]
mod general_tests {
    use super::*;

    #[test]
    fn test_array_sum() {
        let arr = [1, 3, 5, 7, 9];

        assert_eq!(array_sum_v1(&arr), 25);
        assert_eq!(array_sum_v2(&arr, arr.len()), 25);
        assert_eq!(array_sum_v3(&arr, 0, arr.len() - 1), 25);
    }
}
