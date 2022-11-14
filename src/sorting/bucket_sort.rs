/// T(n) = O(n + k)
/// S(n) = O(n + k)
/// where `n` is the number of elements,
/// `k` is the number of buckets used in process

/// `H`: `hasher` 函数回传类型，用来辨识不同的桶
/// `F`: `hasher` 函数自身
/// `T`: 参数类型
pub fn bucket_sort<H, F, T>(arr: &mut [T], hasher: F)
where
    H: Ord,
    F: Fn(&T) -> H,
    T: Ord + Clone,
{
    // 1. Create buckets
    let mut buckets: Vec<Bucket<H, T>> = Vec::new();

    // 2. Scatter
    for value in arr.iter() {
        let hash = hasher(value);

        let value = value.clone();

        match buckets.binary_search_by(|b| b.hash.cmp(&hash)) {
            Ok(i) => buckets[i].values.push(value),
            Err(i) => buckets.insert(i, Bucket::new(hash, value)),
        }
    }

    // 3. Inner sort and gather
    let result: Vec<T> = buckets
        .into_iter()
        .flat_map(|mut b| {
            b.values.sort();
            b.values
        })
        .collect();

    // 4. Copy to original array
    arr.clone_from_slice(&result);
}

struct Bucket<H, T> {
    hash: H,
    values: Vec<T>,
}

impl<H, T> Bucket<H, T> {
    /// Create a new bucket and insert its first value.
    ///
    /// * `hash` - Hash value generated by hasher param of `bucket_sort`.
    /// * `value` - Value to be put in the bucket.
    pub fn new(hash: H, value: T) -> Bucket<H, T> {
        Bucket {
            hash,
            values: vec![value],
        }
    }
}

#[cfg(test)]
mod sorting_tests {
    use super::*;

    #[test]
    fn test_bucket_sort() {
        // unsorted
        let mut arr = vec![5, 7, 2, 3, 1];
        bucket_sort(&mut arr, |t| t / 4);
        assert_eq!(arr, vec![1, 2, 3, 5, 7]);

        // descending
        let mut arr = vec![5, 4, 3, 2, 1];
        bucket_sort(&mut arr, |t| t / 4);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // accending
        let mut arr = vec![1, 2, 3, 4, 5];
        bucket_sort(&mut arr, |t| t / 4);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // one element
        let mut arr: Vec<usize> = vec![2];
        bucket_sort(&mut arr, |t| t / 4);
        assert_eq!(arr, vec![2]);

        // empty
        let mut arr: Vec<usize> = vec![];
        bucket_sort(&mut arr, |t| t / 4);
        assert_eq!(arr, vec![]);
    }
}
