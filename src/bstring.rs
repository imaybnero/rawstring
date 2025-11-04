// bstring::bstring

use std::{
	borrow::{Borrow, BorrowMut},
	ops::{Deref, DerefMut},
	string::FromUtf8Error,
	fmt,
};

use crate::BStr;

/// A mutable, growable sequence of bytes.
/// 
/// `BString` serves as an alternative to Rust's [`String`] type
/// that allows for arbitrary byte sequences,
/// including those that are not valid UTF-8.
/// 
/// `BString` is implemented as a wrapper around, and implements [`Deref`] + [`DerefMut`] to, [`Vec<u8>`].
/// Therefore, all methods available on [`Vec<u8>`] are also available on `BString`.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BString(pub Vec<u8>);

impl BString {
	/// Creates a new [`BString`] from any type that can be converted into a `Vec<u8>`.
	#[inline]
	#[must_use]
	pub fn new<B>(bytes: B) -> Self
	where
		B: Into<Vec<u8>>
	{
		Self::from_bytes(bytes.into())
	}

	/// Returns a reference to the inner byte slice as a [`BStr`].
	#[doc(hidden)]
	#[inline]
	#[must_use]
	pub fn as_bstr(&self) -> &BStr {
		BStr::from_bytes(&self.0)
	}

	/// Returns a mutable reference to the inner byte slice as a mutable [`BStr`].
	#[doc(hidden)]
	#[inline]
	#[must_use]
	pub fn as_mut_bstr(&mut self) -> &mut BStr {
		BStr::from_bytes_mut(&mut self.0)
	}

	/// Wraps the given bytes in a [`BString`].
	#[doc(hidden)]
	#[inline]
	#[must_use]
	pub fn from_bytes(bytes: Vec<u8>) -> Self {
		Self(bytes)
	}
}

impl Deref for BString {
	type Target = Vec<u8>;
	
	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for BString {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl AsRef<[u8]> for BString {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		self.0.as_ref()
	}
}

impl AsRef<BStr> for BString {
	#[inline]
	fn as_ref(&self) -> &BStr {
		self.as_bstr()
	}
}

impl Borrow<[u8]> for BString {
	#[inline]
	fn borrow(&self) -> &[u8] {
		&self.0
	}
}

impl Borrow<BStr> for BString {
	#[inline]
	fn borrow(&self) -> &BStr {
		self.as_bstr()
	}
}

impl BorrowMut<[u8]> for BString {
	#[inline]
	fn borrow_mut(&mut self) -> &mut [u8] {
		&mut self.0
	}
}

impl BorrowMut<BStr> for BString {
	#[inline]
	fn borrow_mut(&mut self) -> &mut BStr {
		self.as_mut_bstr()
	}
}

impl AsMut<[u8]> for BString {
	#[inline]
	fn as_mut(&mut self) -> &mut [u8] {
		self.0.as_mut()
	}
}

impl AsMut<BStr> for BString {
	#[inline]
	fn as_mut(&mut self) -> &mut BStr {
		self.as_mut_bstr()
	}
}

impl fmt::Debug for BString {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_bstr().fmt(f)
	}
}

impl fmt::Display for BString {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_bstr().fmt(f)
	}
}

impl<T: Into<Vec<u8>>> From<T> for BString {
	#[inline]
	fn from(value: T) -> Self {
		Self::new(value)
	}
}

impl TryFrom<BString> for String {
	type Error = FromUtf8Error;

	#[inline]
	fn try_from(this: BString) -> Result<String, FromUtf8Error> {
		String::from_utf8(this.0)
	}
}