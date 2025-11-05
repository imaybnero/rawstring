// raw_str
//! A string type that can hold data which may or may not be valid UTF-8.

#![feature(const_trait_impl, const_convert, const_cmp)]

mod raw_str_imp;
mod raw_string_imp;

#[doc(inline)]
pub use raw_str_imp::RawStr;

#[doc(inline)]
pub use raw_string_imp::RawString;

/// The Unicode replacement character: `�`.
/// 
/// This character replaces invalid or unrepresentable characters
/// when converting [raw strings](RawString) to [UTF-8 strings](String).
pub const UTF8_REPLACEMENT_CHARACTER: char = '�';

/// A macro to create a [`RawStr`] from an expression (usually a string literal).
/// 
/// This macro simply calls [`RawStr::new`] on the provided expression.
/// 
/// # Examples
/// ```
/// # use rawstring::{RawStr, raw_str};
/// // printing a string
/// let raw: &RawStr = raw_str!("Hello, world!");
/// assert_eq!(raw, "Hello, world!");
/// assert_eq!(format!("{}", raw), "Hello, world!");
/// assert_eq!(format!("{:?}", raw), "\"Hello, world!\"");
/// ```
/// ```
/// # use rawstring::{RawStr, raw_str};
/// // printing an invalid utf8 string
/// let raw: &RawStr = raw_str!(&[b'a', 0xFF, b'b']);
/// assert_eq!(raw, b"a\xffb");
/// assert_eq!(format!("{}", raw), "a�b");
/// assert_eq!(format!("{:?}", raw), "\"a\\xffb\"");
/// ```
#[macro_export]
macro_rules! raw_str {
	($expr:expr) => {{
		$crate::RawStr::new($expr)
	}};
}