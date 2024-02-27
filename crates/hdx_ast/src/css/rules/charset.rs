use crate::Atomizable;
use hdx_atom::{atom};
use hdx_lexer::Token;
use hdx_parser::{
	diagnostics::{self},
	expect, unexpected, Parse, Parser, Result as ParserResult, Spanned,
};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum CharsetRule {
	#[atomizable("utf-8")]
	Utf8,
	#[atomizable("us-ascii")]
	UsAscii,
	#[atomizable("iso-8859-1")]
	Iso88591,
	#[atomizable("iso-8859-2")]
	Iso88592,
	#[atomizable("iso-8859-3")]
	Iso88593,
	#[atomizable("iso-8859-4")]
	Iso88594,
	#[atomizable("iso-8859-5")]
	Iso88595,
	#[atomizable("iso-8859-6")]
	Iso88596,
	#[atomizable("iso-8859-7")]
	Iso88597,
	#[atomizable("iso-8859-8")]
	Iso88598,
	#[atomizable("iso-8859-9")]
	Iso88599,
	#[atomizable("iso-8859-10")]
	Iso885910,
	#[atomizable("shift_jis")]
	ShiftJis,
	#[atomizable("euc-jp")]
	EucJp,
	#[atomizable("iso-2022-kr")]
	Iso2022Kr,
	#[atomizable("euc-kr")]
	EucKr,
	#[atomizable("iso-2022-jp")]
	Iso2022Jp,
	#[atomizable("iso-2022-jp-2")]
	Iso2022Jp2,
	#[atomizable("iso-8859-6-e")]
	Iso88596E,
	#[atomizable("iso-8859-6-i")]
	Iso88596I,
	#[atomizable("iso-8859-8-e")]
	Iso88598E,
	#[atomizable("iso-8859-8-i")]
	Iso88598I,
	#[atomizable("gb2312")]
	Gb2312,
	#[atomizable("big5")]
	Big5,
	#[atomizable("koi8-r")]
	Koi8R,
}

impl<'a> Parse<'a> for CharsetRule {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		expect!(parser, Token::AtKeyword(atom!("charset")));
		parser.advance_including_whitespace_and_comments();
		expect!(parser, Token::Whitespace);
		parser.advance();
		let rule = match parser.cur() {
			Token::String(atom) => {
				if let Some(rule) = Self::from_atom(atom.to_ascii_lowercase()) {
					parser.advance();
					rule
				} else {
					Err(diagnostics::UnexpectedCharset(atom, parser.span()))?
				}
			}
			token => unexpected!(parser, token),
		};
		expect!(parser, Token::Semicolon);
		parser.advance();
		Ok(rule.spanned(parser.span()))
	}
}

impl<'a> WriteCss<'a> for CharsetRule {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_str("@charset \"")?;
		self.to_atom().write_css(sink)?;
		sink.write_str("\";")?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<CharsetRule>(), 1);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<CharsetRule>(&allocator, "@charset \"utf-8\";", "@charset \"utf-8\";");
		test_write::<CharsetRule>(&allocator, "@charset \"UTF-8\";", "@charset \"utf-8\";");
	}
}
