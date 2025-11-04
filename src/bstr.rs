// bstring::bstr

use std::{
	cmp::Ordering,
	fmt::{self, Write},
	ops::{Deref, DerefMut},
};

use crate::UTF8_REPLACEMENT_CHARACTER;

/// A transparent wrapper around a slice of bytes.
/// 
/// `BStr` serves as an alternative to Rust's [`str`] type
/// that allows for arbitrary byte sequences,
/// including those that are not valid UTF-8.
/// 
/// `BStr` is implemented as a wrapper around, and implements [`Deref`] + [`DerefMut`] to, `[u8]`.
/// Therefore, all methods available on `[u8]` are also available on `BStr`.
#[repr(transparent)]
#[derive(Eq, Hash)] // PartialEq, PartialOrd, Ord implemented manually
pub struct BStr(pub [u8]);

impl BStr {
	/// Returns a reference to a [`BStr`] from any type
	/// that can be referenced as a byte slice.
	#[inline]
	#[must_use]
	pub const fn new<B>(bytes: &B) -> &Self
	where
		B: ?Sized + [const] AsRef<[u8]>
	{
		Self::from_bytes(bytes.as_ref())
	}

	/// Returns a mutable reference to a [`BStr`] from any type
	/// that can be mutably referenced as a byte slice.
	#[inline]
	#[must_use]
	pub const fn new_mut<B>(b: &mut B) -> &mut Self
	where
		B: ?Sized + [const] AsMut<[u8]>
	{
		Self::from_bytes_mut(b.as_mut())
	}

	/// Reinterprets the given bytes as a [`BStr`].
	#[doc(hidden)]
	#[inline]
	#[must_use]
	pub const fn from_bytes(bytes: &[u8]) -> &Self {
		// SAFETY: BStr is a transparent wrapper over [u8]
		unsafe { &*(bytes as *const [u8] as *const BStr) }
	}

	/// Reinterprets the given mutable bytes slice as a mutable [`BStr`].
	#[doc(hidden)]
	#[inline]
	#[must_use]
	pub const fn from_bytes_mut(bytes: &mut [u8]) -> &mut Self {
		// SAFETY: BStr is a transparent wrapper over [u8]
		unsafe { &mut *(bytes as *mut [u8] as *mut BStr) }
	}
}

impl const Deref for BStr {
	type Target = [u8];
	
	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl const DerefMut for BStr {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl const AsRef<[u8]> for BStr {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		&self.0
	}
}

impl const AsMut<[u8]> for BStr {
	#[inline]
	fn as_mut(&mut self) -> &mut [u8] {
		&mut self.0
	}
}

impl fmt::Debug for BStr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "\"")?;
		for chunk in self.utf8_chunks() {
			for c in chunk.valid().chars() {
				match c {
					'\0' => write!(f, "\\0")?,
					'\x01'..='\x7F' => write!(f, "{}", (c as u8).escape_ascii())?,
					_ => write!(f, "{}", c.escape_debug())?,
				}
			}
			write!(f, "{}", chunk.invalid().escape_ascii())?;
		}
		write!(f, "\"")?;
		Ok(())
	}
}

impl fmt::Display for BStr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fn fmt_no_pad(this: &BStr, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			// formats the bytes as utf8 without any padding
			// invalid utf8 chunks are replaced with the replacement character
			for chunk in this.utf8_chunks() {
				f.write_str(chunk.valid())?;
				if !chunk.invalid().is_empty() {
					f.write_char(UTF8_REPLACEMENT_CHARACTER);
				}
			}
			Ok(())
		}

		if let Some(align) = f.align() {
			// calculate the padding on both sides
			let len: usize = self
				.utf8_chunks()
				.map(|chunk| {
					chunk.valid().chars().count()
					+ if chunk.invalid().is_empty() { 0 } else { 1 }
				})
				.sum();
			let total_padding = f.width()
				.unwrap_or(0)
				.saturating_sub(len);
			let fill = f.fill();
			let (lpad, rpad) = match align {
				fmt::Alignment::Left => (0, total_padding),
				fmt::Alignment::Right => (total_padding, 0),
				fmt::Alignment::Center => {
					let half = total_padding / 2;
					(half, half + total_padding % 2)
				}
			};

			// write the padding and the formatted bytes
			for _ in 0..lpad {
				f.write_char(fill);
			}
			fmt_no_pad(self, f)?;
			for _ in 0..rpad {
				f.write_char(fill);
			}

			Ok(())
		} else {
			// no padding needed
			// directly format the bytes
			fmt_no_pad(self, f)
		}
	}
}

impl<T: ?Sized + [const] AsRef<[u8]>> const PartialEq<T> for BStr {
	fn eq(&self, other: &T) -> bool {
		&self.0 == other.as_ref()
	}
}

impl<T: ?Sized + AsRef<[u8]>> PartialOrd<T> for BStr {
	fn partial_cmp(&self, other: &T) -> Option<Ordering> {
		Some(self.0.cmp(other.as_ref()))
	}
}

impl Ord for BStr {
	fn cmp(&self, other: &Self) -> Ordering {
		self.0.cmp(&other.0)
	}
}