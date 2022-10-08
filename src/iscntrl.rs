//! Rust implementation of C library function `iscntrl`
//!
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::CInt;

/// Rust implementation of C library function `iscntrl`
#[no_mangle]
pub unsafe extern "C" fn iscntrl(c: CInt) -> CInt {
	if (c >= 0x00 && c <= 0x1f) || c == 0x7F {
		1
	} else {
		0
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn works() {
		let control_chars = "0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 127";

		for c in CInt::from(-1000)..CInt::from(1000) {
			let expected = control_chars
				.split(' ')
				.map(|s| s.parse().unwrap())
				.any(|ctrl| c == ctrl);
			let actual = unsafe { iscntrl(c) } != 0;
			assert_eq!(expected, actual, "Incorrect result for 0x{:x}", c);
		}
	}
}
