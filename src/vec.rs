//! Utility methods for Vec.

/// ```
/// use contest::vec;
/// let mut is = vec![2,3,1];
/// assert!(vec::next_permutation(&mut is));
/// assert_eq!(vec![3,1,2], is);
/// assert!(vec::next_permutation(&mut is));
/// assert_eq!(vec![3,2,1], is);
/// assert!(!vec::next_permutation(&mut is));
/// assert_eq!(vec![1,2,3], is);
///
/// let mut is = (0..3).collect();
/// let mut count = 0;
/// loop {
///     count+=1;
///     // Do something using is
///     if !vec::next_permutation(&mut is) {
///         break;
///     }
/// }
/// assert_eq!(6, count);
///
/// let mut is = vec![1,2,1];
/// assert!(vec::next_permutation(&mut is));
/// assert_eq!(vec![2,1,1], is);
/// assert!(!vec::next_permutation(&mut is));
/// assert_eq!(vec![1,1,2], is);
/// ```
pub fn next_permutation(is: &mut Vec<i32>) -> bool {
    let n = is.len();
    for i in (1..n).rev() {
        let i = i as usize;
        if is[i - 1] < is[i] {
            let mut j = n - 1;
            while is[i - 1] >= is[j] {
                j -= 1;
            }
            is.swap(i - 1, j);
            reverse(is, i, n);
            return true;
        }
    }
    is.reverse();
    false
}

fn reverse(v: &mut Vec<i32>, from_inclusive: usize, to_exclusive: usize) {
    let mut i = from_inclusive;
    let mut j = to_exclusive - 1;
    while i < j {
        v.swap(i, j);
        i += 1;
        j -= 1;
    }
}
