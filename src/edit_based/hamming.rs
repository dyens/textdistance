use std::cmp::Eq;
/// Compute the Hamming distance between the two or more sequences.

/// Calculates the Hamming distance between two strings of equal length.
///
/// # Hamming distance
/// The [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance) between two strings of equal length is the number of positions at
/// which two strings are different.
/// This returns an error of type DistanceError::InvalidArgs if the string arguments do not have equal length.
/// This implementation does fully support unicode strings.
///
/// ## Complexity
/// Time complexity: O(n)
/// Space complexity: O(1)
///
/// ## Examples
/// ```
/// use distance::*;
///
/// // Hamming distance
/// let distance = hamming("karolin", "kathrin").unwrap();   
/// assert_eq!(3, distance);
/// ```
///
///https://github.com/life4/textdistance/blob/master/textdistance/algorithms/edit_based.py#L33
///https://github.com/mbrlabs/distance/blob/master/src/hamming.rs

pub fn hamming<T>(s1: T, s2: T) -> u32
where
    T: IntoIterator,
    T::Item: Eq,
{
    let mut distance = 0;
    for (s1_el, s2_el) in s1.into_iter().zip(s2) {
        if s1_el != s2_el {
            distance += 1;
        }
    }
    distance
}

#[cfg(test)]
mod tests {
    use super::hamming;

    #[test]
    fn basic() {
        assert_eq!(0, hamming("sitting".chars(), "sitting".chars()));
        assert_eq!(7, hamming("abcdefg".chars(), "hijklmn".chars()));
        assert_eq!(3, hamming("karolin".chars(), "kathrin".chars()));
        assert_eq!(4, hamming("hello".chars(), "world".chars()));
        assert_eq!(1, hamming("Rust".chars(), "rust".chars()));
    }
}
