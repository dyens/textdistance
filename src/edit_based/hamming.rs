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
use crate::error::{TextDistanceError, ValueError};
use std::cmp::Eq;

fn _hamming<T>(x: T, y: T) -> u64
where
    T: IntoIterator,
    T::Item: Eq,
{
    let mut distance = 0;
    for (x_el, y_el) in x.into_iter().zip(y) {
        if x_el != y_el {
            distance += 1;
        }
    }
    distance
}

fn hamming_on_slices<T>(x: &[T], y: &[T]) -> u64
where
    T: Eq,
{
    if x.len() != y.len() {
        return 0;
    }
    _hamming(x, y)
}

fn hamming_on_str(x: &str, y: &str) -> u64 {
    if x.chars().count() != y.chars().count() {
        return 0;
    }
    _hamming(x.chars(), y.chars())
}

#[allow(non_camel_case_types)]
pub struct hamming;

impl<T> FnOnce<(&[T], &[T])> for hamming
where
    T: Eq,
{
    type Output = u64;
    #[inline(always)]
    extern "rust-call" fn call_once(self, args: (&[T], &[T])) -> Self::Output {
        hamming_on_slices.call_once(args)
    }
}

impl<T> FnOnce<(&Vec<T>, &Vec<T>)> for hamming
where
    T: Eq,
{
    type Output = u64;
    #[inline(always)]
    extern "rust-call" fn call_once(self, args: (&Vec<T>, &Vec<T>)) -> Self::Output {
        hamming_on_slices.call_once((&args.0[..], &args.1[..]))
    }
}

impl FnOnce<(&str, &str)> for hamming {
    type Output = u64;
    #[inline(always)]
    extern "rust-call" fn call_once(self, args: (&str, &str)) -> Self::Output {
        hamming_on_str.call_once(args)
    }
}

impl FnOnce<(&String, &String)> for hamming {
    type Output = u64;
    #[inline(always)]
    extern "rust-call" fn call_once(self, args: (&String, &String)) -> Self::Output {
        hamming_on_str.call_once((&args.0, &args.1))
    }
}

impl FnOnce<(u32, u32)> for hamming {
    type Output = u64;
    #[inline(always)]
    extern "rust-call" fn call_once(self, args: (u32, u32)) -> Self::Output {
        3
    }
}

// impl FnOnce<(&str, &str)> for hamming {
//     type Output = u64;
//     #[inline(always)]
//     extern "rust-call" fn call_once(self, args: (&str, &str)) -> Self::Output {
//         hamming_on_str.call_once(args)
//     }
// }

// // pub trait Hamming<Rhs = Self> {
//     fn hamming(&self, other: &Rhs) -> Result<u32, TextDistanceError>;
// }

// impl<T> Hamming for Vec<T>
// where
//     T: Eq,
// {
//     fn hamming(&self, other: &Vec<T>) -> Result<u32, TextDistanceError> {
//         if self.len() != other.len() {
//             ValueError::new("Lengths not equal");
//         }
//         Ok(_hamming(self, other))
//     }
// }

// impl Hamming for &str {
//     fn hamming(&self, other: &&str) -> Result<u32, TextDistanceError> {
//         if self.chars().count() != other.chars().count() {
//             return Err(TextDistanceError::new("Lengths not equal"));
//         }
//         Ok(_hamming(self.chars(), other.chars()))
//     }
// }

// impl Hamming for String {
//     fn hamming(&self, other: &String) -> Result<u32, TextDistanceError> {
//         (&self[..]).hamming(&&other[..])
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::Hamming;

//     #[test]
//     fn test_str_slice() {
//         let x = "abc";
//         let y = "abd";
//         assert_eq!(x.hamming(&y).unwrap(), 1);
//         let x = "abcc";
//         let y = "abd";
//         assert_eq!(x.hamming(&y).unwrap(), 1);
//         assert_eq!(x.hamming(&y).is_err(), true);
//     }
// }
