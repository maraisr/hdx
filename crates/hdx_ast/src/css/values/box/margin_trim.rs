use hdx_atom::atom;
use hdx_lexer::Token;
use hdx_parser::{unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{bitmask, Atomizable, Value};

// https://drafts.csswg.org/css-box-4/#propdef-margin-trim
#[derive(Value, Atomizable, Default)]
#[bitmask(u8)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum MarginTrim {
	#[default]
	None,
	Block,
	Inline,
	BlockStart,
	BlockEnd,
	InlineStart,
	InlineEnd,
}

impl<'a> Parse<'a> for MarginTrim {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut value = Self::none();
		loop {
			if value.is_all() {
				break;
			}
			match parser.cur() {
				Token::Ident(atom) => match atom.to_ascii_lowercase() {
					atom!("block") => value |= MarginTrim::Block,
					atom!("inline") => value |= MarginTrim::Inline,
					atom!("block-start") => value |= MarginTrim::BlockStart,
					atom!("block-end") => value |= MarginTrim::BlockEnd,
					atom!("inline-start") => value |= MarginTrim::InlineStart,
					atom!("inline-end") => value |= MarginTrim::InlineEnd,
					atom => unexpected_ident!(parser, atom),
				},
				token => unexpected!(parser, token),
			}
			parser.advance();
			if value == Self::None || value == Self::Block || value == Self::Inline {
				break;
			}
		}
		if value.is_none() {
			unexpected!(parser);
		}
		Ok(value)
	}
}

impl<'a> WriteCss<'a> for MarginTrim {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		if self.contains(Self::BlockStart) {
			atom!("block-start").write_css(sink)?;
		}
		if self.contains(Self::BlockEnd) {
			if self.intersects(Self::BlockStart) {
				sink.write_char(' ')?;
			}
			atom!("block-end").write_css(sink)?;
		}
		if self.contains(Self::InlineStart) {
			if self.intersects(Self::BlockStart | Self::BlockEnd) {
				sink.write_char(' ')?;
			}
			atom!("inline-start").write_css(sink)?;
		}
		if self.contains(Self::InlineEnd) {
			if self.intersects(Self::BlockStart | Self::BlockEnd | Self::InlineStart) {
				sink.write_char(' ')?;
			}
			atom!("inline-end").write_css(sink)?;
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(MarginTrim, 1);
	}
}
