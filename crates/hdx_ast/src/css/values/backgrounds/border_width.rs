use crate::{css::units::LineWidth, macros::*, Parsable, Value, Writable};

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderTopWidth(#[parsable(FromToken)] LineWidth);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderBottomWidth(#[parsable(FromToken)] LineWidth);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderLeftWidth(#[parsable(FromToken)] LineWidth);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderRightWidth(#[parsable(FromToken)] LineWidth);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderBlockStartWidth(#[parsable(FromToken)] LineWidth);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderBlockEndWidth(#[parsable(FromToken)] LineWidth);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderInlineStartWidth(#[parsable(FromToken)] LineWidth);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderInlineEndWidth(#[parsable(FromToken)] LineWidth);

// https://drafts.csswg.org/css-backgrounds/#propdef-border-block-width
#[derive(Value, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderBlockWidth(pub BorderBlockStartWidth, pub BorderBlockEndWidth);

parse_logical_sides!(BorderBlockWidth, LineWidth, BorderBlockStartWidth, BorderBlockEndWidth);
write_logical_sides!(BorderBlockWidth);

// https://drafts.csswg.org/css-backgrounds/#propdef-border-inline-width
#[derive(Value, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderInlineWidth(pub BorderInlineStartWidth, pub BorderInlineEndWidth);

parse_logical_sides!(BorderInlineWidth, LineWidth, BorderInlineStartWidth, BorderInlineEndWidth);
write_logical_sides!(BorderInlineWidth);

// https://drafts.csswg.org/css-backgrounds/#propdef-border-width
#[derive(Value, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderWidth(
	pub BorderBlockStartWidth,
	pub BorderInlineEndWidth,
	pub BorderBlockEndWidth,
	pub BorderInlineStartWidth,
);

parse_rect!(
	BorderWidth,
	LineWidth,
	BorderBlockStartWidth,
	BorderInlineEndWidth,
	BorderBlockEndWidth,
	BorderInlineStartWidth
);
write_rect!(BorderWidth);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BorderWidth, 32);
		assert_size!(BorderBlockStartWidth, 8);
		assert_size!(BorderBlockEndWidth, 8);
		assert_size!(BorderInlineStartWidth, 8);
		assert_size!(BorderInlineEndWidth, 8);
		assert_size!(BorderTopWidth, 8);
		assert_size!(BorderRightWidth, 8);
		assert_size!(BorderLeftWidth, 8);
		assert_size!(BorderBottomWidth, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BorderLeftWidth, "medium");
		assert_parse!(BorderBlockWidth, "1px 1px");
		assert_parse!(BorderBlockWidth, "1px 2px");
		assert_parse!(BorderInlineWidth, "1px 1px");
		assert_parse!(BorderInlineWidth, "1px 2px");
		assert_parse!(BorderWidth, "1px 1px 1px 1px");
		assert_parse!(BorderWidth, "1px 2px 1px 2px");
		assert_parse!(BorderWidth, "1px 2px 3px 4px");
		assert_parse!(BorderWidth, "thick medium thin 0");
	}

	#[test]
	fn test_minify() {
		assert_minify!(BorderWidth, "1px 1px", "1px");
		assert_minify!(BorderWidth, "1px 1px 1px", "1px");
		assert_minify!(BorderWidth, "1px 1px 1px 1px", "1px");
		assert_minify!(BorderWidth, "thick medium thick medium", "thick medium");
		assert_minify!(BorderWidth, "1px 2px 1px", "1px 2px");
		assert_minify!(BorderWidth, "1px 2px medium 2px", "1px 2px medium");
	}
}
