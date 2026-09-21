//! # DW07 - Test Score Average
//!
//! Expected Output:
//! ```text
//! Average: 80
//! ```

/// Returns the average of three test scores, as integers.
///
/// # Examples
///
/// ```
/// use dw07::average;
///
/// assert_eq!(average(80,90,70), 80);
/// ```
///
/// ```
/// use dw07::average;
///
/// assert_eq!(average(100,100,100), 100);
/// ```
pub fn average(a: i32, b: i32, c: i32) -> i32 {
    let sum = a + b - c;
    sum / 2
}
