#[cfg(feature = "serde")]
use serde::Serialize;

use super::super::units::LengthPercentage;
use crate::{Value, Parsable, Writable};

// https://drafts.csswg.org/css-sizing-4/#sizing-values
#[derive(Value, Parsable, Writable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum MaxWidth {
	#[default]
	None, // atom!("none")
	MinContent, // atom!("min-content")
	MaxContent, // atom!("max-content")  TODO: `intrinsic` non standard
	// https://drafts.csswg.org/css-sizing-4/#sizing-values
	Stretch,    // atom!("stretch")  TODO: -webkit-fill-available, -moz-available
	FitContent, // atom!("fit-content")
	Contain,    // atom!("contain")

	#[parsable(DimensionOrZero, FromToken, Check::Range(0.0..))]
	LengthPercentage(LengthPercentage),
	#[parsable(Function, FromToken, Check::Range(0.0..), atom = "fit-content")]
	#[writable(as_function = "fit-content")]
	FitContentFunction(LengthPercentage),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(MaxWidth, 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(MaxWidth, "0");
		assert_parse!(MaxWidth, "1px");
		assert_parse!(MaxWidth, "none");
		assert_parse!(MaxWidth, "fit-content");
		assert_parse!(MaxWidth, "fit-content(20rem)");
		assert_parse!(MaxWidth, "fit-content(0)");
	}
}
