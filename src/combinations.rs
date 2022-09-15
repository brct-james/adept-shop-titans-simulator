/// Alternative way to count combinations of size r in n
pub fn count_combinations(n: i64, r: i64) -> i64 {
    if r > n {
        0
    } else {
        (1..=r.min(n - r)).fold(1, |acc, val| acc * (n - val + 1) / val)
    }
}

/// Yield the items of the single combination that would be at the provided 0-based index in a lexicographically sorted list of combinations of choices of r items from n items [0, n), given the combinations were sorted in descending order. Yields in descending order.
pub fn iter_combination(mut index: i64, mut n: i64, mut r: i64) -> Vec<i64> {
    let mut res: Vec<i64> = Default::default();
    if index < 0 || index >= count_combinations(n, r) {
        return res;
    }
    n -= 1;
    for _ in 0..r {
        while count_combinations(n, r) > index {
            n -= 1;
        }
        res.push(n);
        index -= count_combinations(n, r);
        n -= 1;
        r -= 1;
    }
    return res;
}

// def iterCombination(index, n, k):
//     '''Yields the items of the single combination that would be at the provided
//     (0-based) index in a lexicographically sorted list of combinations of choices
//     of k items from n items [0,n), given the combinations were sorted in
//     descending order. Yields in descending order.
//     '''
//     if index < 0 or index >= choose(n, k):
//         return
//     n -= 1
//     for i in range(k):
//         while choose(n, k) > index:
//             n -= 1
//         yield n
//         index -= choose(n, k)
//         n -= 1
//         k -= 1

// /// Returns the number of ways to choose k items from n items
// pub fn _choose(mut n: usize, mut k: usize) -> usize {
//     let reflect = n - k;
//     if k > reflect {
//         if k > n {
//             return 0;
//         }
//         k = reflect;
//     }
//     if k == 0 {
//         return 1;
//     }
//     for n_minus_iplus1 in ((n - k)..(n - 1)).rev() {
//         // was previously ((n - 1)..(n - k)).rev()) but didn't work - maybe because of type usize? Perhaps i64 would be better?
//         n = n * n_minus_iplus1; // i
//     }
//     return n;
// }
