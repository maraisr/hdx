use crate::{css::units::CSSFloat, Parsable, Writable};

#[derive(Parsable, Writable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize),
	serde(tag = "type", content = "value", rename_all = "kebab-case")
)]
pub enum Zoom {
	#[default]
	Normal, // atom!("normal")
	Reset, // atom!("reset")
	#[parsable(Number)]
	Number(CSSFloat),
	#[parsable(Dimension, atom = "%")]
	#[writable(suffix = "%")]
	#[cfg_attr(feature = "serde", serde(rename = "%"))]
	Percent(CSSFloat),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Zoom, 8);
	}
}
