// bstring

#![allow(unused)]
#![feature(const_trait_impl, const_convert, const_cmp)]

mod bstr;
mod bstring;

#[doc(inline)]
pub use bstr::BStr;

#[doc(inline)]
pub use bstring::BString;

/// The Unicode replacement character: `�`.
/// 
/// This character is substituted for invalid or unrepresentable characters
/// when converting byte strings to UTF-8 strings.
pub const UTF8_REPLACEMENT_CHARACTER: char = '\u{FFFD}';

/// A macro to create a [`BStr`] from an expression (usually a string literal).
/// 
/// # Examples
/// ```
/// # use bstring::{BStr, bstr};
/// // printing a string
/// let bstr: &BStr = bstr!("Hello, world!");
/// assert_eq!(bstr, "Hello, world!");
/// assert_eq!(format!("{}", bstr), "Hello, world!");
/// assert_eq!(format!("{:?}", bstr), "\"Hello, world!\"");
/// ```
/// 
/// ```
/// # use bstring::{BStr, bstr};
/// // printing an invalid utf8 string
/// let bstr: &BStr = bstr!(&[b'a', 0xFF, b'b']);
/// assert_eq!(bstr, b"a\xffb");
/// assert_eq!(format!("{}", bstr), "a�b");
/// assert_eq!(format!("{:?}", bstr), "\"a\\xffb\"");
/// ```
#[macro_export]
macro_rules! bstr {
	($expr:expr) => {{
		$crate::BStr::new($expr)
	}};
}