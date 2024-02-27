use hdx_atom::{atom, Atom};
use hdx_lexer::Token;
use hdx_parser::{unexpected, unexpected_ident, FromToken, Parse, Parser, Result as ParserResult, Spanned};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Value, Writable};
use smallvec::{SmallVec, smallvec};

// https://drafts.csswg.org/css-animations-2/#animation-duration
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct FontFamily(pub SmallVec<[Spanned<SingleFontFamily>; 1]>);

#[derive(Writable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum SingleFontFamily {
	#[writable(String)]
	Named(Atom),
	Generic(Atom),
	// Generic Font Families
	Serif, // atom!("serif")
	#[default]
	SansSerif, // atom!("sans-serif")
	Cursive, // atom!("cursive")
	Fantasy, // atom!("fantasy")
	Monospace, // atom!("monospace")
	SystemUi, // atom!("system-ui")
	Math, // atom!("math")
	Fangsong,// atom!("fangsong")
	Kai,// atom!("kai")
	Nastaliq,// atom!("nastaliq")
	UiSerif,// atom!("ui-serif")
	UiMonospace,// atom!("ui-monospace")
	UiRounded,// atom!("ui-rounded")
	// <system-family-name>
	Caption,// atom!("caption")
	Icon,// atom!("icon")
	Menu,// atom!("menu")
	MessageBox,// atom!("message-box")
	SmallCaption,// atom!("small-caption")
	StatusBar,// atom!("status-bar")
}

impl<'a> Value for FontFamily {}

impl<'a> Parse<'a> for SingleFontFamily {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		let value = match parser.cur() {
			Token::Ident(ident) => {
				parser.advance();
				match ident.to_ascii_lowercase() {
					atom!("serif") => Self::Serif,
					atom!("sans-serif") => Self::SansSerif,
					atom!("cursive") => Self::Cursive,
					atom!("fantasy") => Self::Fantasy,
					atom!("monospace") => Self::Monospace,
					atom!("system-ui") => Self::SystemUi,
					atom!("math") => Self::Math,
					atom!("ui-serif") => Self::UiSerif,
					atom!("ui-monospace") => Self::UiMonospace,
					atom!("ui-rounded") => Self::UiRounded,
					atom!("caption") => Self::Caption,
					atom!("icon") => Self::Icon,
					atom!("menu") => Self::Menu,
					atom!("message-box") => Self::MessageBox,
					atom!("small-caption") => Self::SmallCaption,
					atom!("status-bar") => Self::StatusBar,
					_ => Self::Named(ident),
				}
			},
			Token::Function(atom!("generic")) => {
				parser.advance();
				match parser.cur() {
					Token::Ident(ident) => Self::Generic(ident),
					token => unexpected!(parser, token)
				}
			},
			Token::String(atom) => {
				parser.advance();
				Self::Named(atom)
			}
			token => unexpected!(parser, token),
		};
		Ok(value.spanned(span.end(parser.pos())))
	}
}

impl<'a> Parse<'a> for FontFamily {
    fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		let mut values = smallvec![];
		loop {
			let value = SingleFontFamily::parse(parser)?;
			values.push(value);
			match parser.cur() {
				Token::Comma => {
					parser.advance();
				}
				_ => {
					break;
				}
			}
		}
		Ok(FontFamily(values).spanned(span.end(parser.pos())))
	}
}

impl<'a> WriteCss<'a> for FontFamily {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		let mut iter = self.0.iter().peekable();
		while let Some(time) = iter.next() {
			time.write_css(sink)?;
			if iter.peek().is_some() {
				sink.write_char(',')?;
				sink.write_trivia_char(' ')?;
			}
		}
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
		assert_eq!(size_of::<FontFamily>(), 40);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<FontFamily>(&allocator, "serif", "serif");
		test_write::<FontFamily>(&allocator, "Arial, sans-serif", "\"Arial\",sans-serif");
		test_write::<FontFamily>(&allocator, "'Gill Sans MS', Arial, system-ui, sans-serif", "\"Gill Sans MS\",\"Arial\",system-ui,sans-serif");
	}
}
