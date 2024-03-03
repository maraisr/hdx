use smallvec::{smallvec, SmallVec};

use hdx_atom::{atom, Atom};
use hdx_lexer::Token;
use hdx_parser::{
	diagnostics, expect, expect_ignore_case, match_ident_ignore_case, peek, unexpected, unexpected_ident, AtRule, RuleGroup,
	FromToken, Parse, Parser, Result as ParserResult, Spanned, Vec,
};
use hdx_writer::{CssWriter, OutputOption, Result as WriterResult, WriteCss};

use crate::css::stylerule::StyleRule;

mod features;
use features::*;

// https://drafts.csswg.org/mediaqueries-4/
#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct MediaRule<'a> {
	pub query: Spanned<MediaQueryList>,
	pub rules: Spanned<MediaRules<'a>>,
}

// https://drafts.csswg.org/css-conditional-3/#at-ruledef-media
impl<'a> Parse<'a> for MediaRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect_ignore_case!(parser, AtKeyword, atom!("media"));
		let span = parser.span();
		match Self::parse_at_rule(parser)? {
			(Some(query), Some(rules)) => Ok(Self { query, rules }),
			(Some(_), None) => Err(diagnostics::MissingAtRuleBlock(span.end(parser.pos())))?,
			(None, Some(_)) => Err(diagnostics::MissingAtRulePrelude(span.end(parser.pos())))?,
			(None, None) => Err(diagnostics::MissingAtRulePrelude(span.end(parser.pos())))?,
		}
	}
}

impl<'a> AtRule<'a> for MediaRule<'a> {
	type Prelude = MediaQueryList;
	type Block = MediaRules<'a>;
}

impl<'a> WriteCss<'a> for MediaRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		if !sink.can_output(OutputOption::RedundantRules) && self.rules.node.0.is_empty() {
			return Ok(());
		}
		sink.write_char('@')?;
		atom!("media").write_css(sink)?;
		if matches!(self.query.node.0.first(), Some(Spanned { node: MediaQuery::Condition(_), .. })) {
			sink.write_whitespace()?;
		} else {
			sink.write_char(' ')?;
		}
		self.query.write_css(sink)?;
		sink.write_whitespace()?;
		sink.write_char('{')?;
		sink.write_newline()?;
		sink.indent();
		self.rules.write_css(sink)?;
		sink.write_newline()?;
		sink.dedent();
		sink.write_char('}')?;
		Ok(())
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MediaRules<'a>(pub Vec<'a, Spanned<StyleRule<'a>>>);

impl<'a> Parse<'a> for MediaRules<'a> {
    fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
        Ok(Self(Self::parse_rules(parser)?))
    }
}

impl<'a> RuleGroup<'a> for MediaRules<'a> {
	type Rule = StyleRule<'a>;
}

impl<'a> WriteCss<'a> for MediaRules<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		let mut rules = self.0.iter().peekable();
		while let Some(rule) = rules.next() {
			rule.write_css(sink)?;
			if rules.peek().is_some() {
				sink.write_newline()?;
			}
		}
		Ok(())
    }
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MediaQueryList(pub SmallVec<[Spanned<MediaQuery>; 1]>);

impl<'a> Parse<'a> for MediaQueryList {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut queries = smallvec![];
		loop {
			let query = MediaQuery::parse_spanned(parser)?;
			queries.push(query);
			if matches!(parser.cur(), Token::Comma) {
				parser.advance();
			} else {
				return Ok(Self(queries));
			}
		}
	}
}

impl<'a> WriteCss<'a> for MediaQueryList {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		let mut queries = self.0.iter().peekable();
		while let Some(query) = queries.next() {
			query.write_css(sink)?;
			if queries.peek().is_some() {
				sink.write_char(',')?;
				sink.write_whitespace()?;
			}
		}
		Ok(())
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum MediaQuery {
	Condition(MediaCondition),
	Typed(MediaType),
	NotTyped(MediaType),
	OnlyTyped(MediaType),
	TypedCondition(MediaType, MediaCondition),
	NotTypedCondition(MediaType, MediaCondition),
	OnlyTypedCondition(MediaType, MediaCondition),
}

impl<'a> Parse<'a> for MediaQuery {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut not = false;
		let mut only = false;
		let mut media_type = None;
		match parser.cur() {
			Token::Ident(ident) => match ident.to_ascii_lowercase() {
				atom!("not") => {
					parser.advance();
					not = true;
				}
				atom!("only") => {
					parser.advance();
					only = true;
				}
				_ => {
					if let Some(ty) = MediaType::from_token(parser.cur()) {
						parser.advance();
						media_type = Some(ty);
					} else {
						unexpected_ident!(parser, ident);
					}
				}
			},
			Token::LeftParen => {
				return Ok(Self::Condition(MediaCondition::parse(parser)?));
			}
			token => {
				unexpected!(parser, token);
			}
		}
		match parser.cur() {
			Token::Ident(ident) if only || not => {
				if let Some(ty) = MediaType::from_token(parser.cur()) {
					parser.advance();
					media_type = Some(ty);
				} else {
					unexpected_ident!(parser, ident)
				}
			}
			Token::Ident(ident) if media_type.is_some() && matches!(ident.to_ascii_lowercase(), atom!("and")) => {
				// Must not advance because we need "and" to be consumed by MediaCondition
				return Ok(Self::TypedCondition(media_type.unwrap(), MediaCondition::parse(parser)?));
			}
			token => {
				if let Some(mt) = media_type {
					return Ok(Self::Typed(mt));
				} else {
					unexpected!(parser, token)
				}
			}
		}
		match parser.cur() {
			Token::Ident(ident) if matches!(ident.to_ascii_lowercase(), atom!("and")) => {
				// Must not advance because we need "and" to be consumed by MediaCondition
				if only {
					Ok(Self::OnlyTypedCondition(media_type.unwrap(), MediaCondition::parse(parser)?))
				} else if not {
					Ok(Self::NotTypedCondition(media_type.unwrap(), MediaCondition::parse(parser)?))
				} else {
					unexpected_ident!(parser, ident)
				}
			}
			_ if only => Ok(Self::OnlyTyped(media_type.unwrap())),
			_ if not => Ok(Self::NotTyped(media_type.unwrap())),
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for MediaQuery {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Condition(mc) => mc.write_css(sink),
			Self::Typed(mt) => mt.write_css(sink),
			Self::NotTyped(mt) => {
				atom!("not").write_css(sink)?;
				sink.write_whitespace()?;
				mt.write_css(sink)
			}
			Self::OnlyTyped(mt) => {
				atom!("only").write_css(sink)?;
				sink.write_whitespace()?;
				mt.write_css(sink)
			}
			Self::TypedCondition(mt, mc) => {
				mt.write_css(sink)?;
				sink.write_whitespace()?;
				mc.write_css(sink)
			}
			Self::NotTypedCondition(mt, mc) => {
				atom!("not").write_css(sink)?;
				sink.write_char(' ')?;
				mt.write_css(sink)?;
				sink.write_whitespace()?;
				mc.write_css(sink)
			}
			Self::OnlyTypedCondition(mt, mc) => {
				atom!("only").write_css(sink)?;
				sink.write_char(' ')?;
				mt.write_css(sink)?;
				sink.write_whitespace()?;
				mc.write_css(sink)
			}
		}
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub enum MediaCondition {
	Is(MediaFeature),
	Not(MediaFeature),
	And(SmallVec<[MediaFeature; 4]>),
	Or(SmallVec<[MediaFeature; 4]>),
}

impl<'a> Parse<'a> for MediaCondition {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match parser.cur() {
			Token::LeftParen => {
				if peek!(parser, Token::LeftParen) {
					todo!()
				} else {
					Ok(Self::Is(MediaFeature::parse(parser)?))
				}
			}
			Token::Ident(ident) => match ident.to_ascii_lowercase() {
				atom!("and") => {
					let mut features = smallvec![];
					loop {
						expect_ignore_case!(parser, atom!("and"));
						parser.advance();
						features.push(MediaFeature::parse(parser)?);
						if !match_ident_ignore_case!(parser, atom!("and")) {
							return Ok(Self::And(features));
						}
					}
				}
				atom!("or") => {
					let mut features = smallvec![];
					loop {
						expect_ignore_case!(parser, atom!("or"));
						parser.advance();
						features.push(MediaFeature::parse(parser)?);
						if !match_ident_ignore_case!(parser, atom!("or")) {
							return Ok(Self::And(features));
						}
					}
				}
				atom!("not") => Ok(Self::Not(MediaFeature::parse(parser)?)),
				_ => unexpected_ident!(parser, ident),
			},
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for MediaCondition {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Is(feature) => feature.write_css(sink),
			Self::Not(feature) => {
				atom!("not").write_css(sink)?;
				sink.write_whitespace()?;
				feature.write_css(sink)
			}
			Self::And(features) => {
				let mut iter = features.iter().peekable();
				while let Some(feature) = iter.next() {
					atom!("and").write_css(sink)?;
					sink.write_whitespace()?;
					feature.write_css(sink)?;
					if iter.peek().is_some() {
						sink.write_whitespace()?;
					}
				}
				Ok(())
			}
			Self::Or(features) => {
				for feature in features.iter() {
					sink.write_char(' ')?;
					atom!("or").write_css(sink)?;
					sink.write_char(' ')?;
					feature.write_css(sink)?;
				}
				Ok(())
			}
		}
	}
}

macro_rules! media_features {
	( $($name: ident($typ: ident): atom!($atom: tt),)+ ) => {
		// https://drafts.csswg.org/mediaqueries-5/#media-descriptor-table
		#[derive(Debug, PartialEq, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
		pub enum MediaFeature {
			$($name($typ),)+
		}

		impl<'a> Parse<'a> for MediaFeature {
			fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
				expect!(parser, Token::LeftParen);
				parser.advance();
				let value = match parser.cur() {
					Token::Ident(ident) => match ident.to_ascii_lowercase() {
						$(atom!($atom) => Self::$name($typ::parse(parser)?),)+
						_ => unexpected_ident!(parser, ident),
					},
					token => unexpected!(parser, token),
				};
				expect!(parser, Token::RightParen);
				parser.advance();
				Ok(value)
			}
		}

		impl<'a> WriteCss<'a> for MediaFeature {
			fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
				sink.write_char('(')?;
				match self {
					$(Self::$name(f) => f.write_css(sink)?,)+
				}
				sink.write_char(')')
			}
		}
	}
}

media_features!(
	AnyHover(AnyHoverMediaFeature): atom!("any-hover"),
	AnyPointer(AnyPointerMediaFeature): atom!("any-pointer"),
	AspectRatio(AspectRatioMediaFeature): atom!("aspect-ratio"),
	Color(ColorMediaFeature): atom!("color"),
	ColorGamut(ColorGamutMediaFeature): atom!("color-gamut"),
	ColorIndex(ColorIndexMediaFeature): atom!("color-index"),
	DeviceAspectRatio(DeviceAspectRatioMediaFeature): atom!("device-aspect-ratio"),
	DeviceHeight(DeviceHeightMediaFeature): atom!("device-height"),
	DeviceWidth(DeviceWidthMediaFeature): atom!("device-width"),
	DisplayMode(DisplayModeMediaFeature): atom!("display-mode"),
	DynamicRange(DynamicRangeMediaFeature): atom!("dynamic-range"),
	EnvironmentBlending(EnvironmentBlendingMediaFeature): atom!("environment-blending"),
	ForcedColors(ForcedColorsMediaFeature): atom!("forced-colors"),
	Grid(GridMediaFeature): atom!("grid"),
	Height(HeightMediaFeature): atom!("height"),
	// HorizontalViewportSegments(HorizontalViewportSegmentsMediaFeature): atom!("horizontal-viewport-segments"),
	Hover(HoverMediaFeature): atom!("hover"),
	InvertedColors(InvertedColorsMediaFeature): atom!("inverted-colors"),
	Monochrome(MonochromeMediaFeature): atom!("monochrome"),
	NavControls(NavControlsMediaFeature): atom!("nav-controls"),
	Orientation(OrientationMediaFeature): atom!("orientation"),
	OverflowBlock(OverflowBlockMediaFeature): atom!("overflow-block"),
	OverflowInline(OverflowInlineMediaFeature): atom!("overflow-inline"),
	Pointer(PointerMediaFeature): atom!("pointer"),
	PrefersColorScheme(PrefersColorSchemeMediaFeature): atom!("prefers-color-scheme"),
	PrefersContrast(PrefersContrastMediaFeature): atom!("prefers-contrast"),
	PrefersReducedData(PrefersReducedDataMediaFeature): atom!("prefers-reduced-data"),
	PrefersReducedMotion(PrefersReducedMotionMediaFeature): atom!("prefers-reduced-motion"),
	PrefersReducedTransparency(PrefersReducedTransparencyMediaFeature): atom!("prefers-reduced-transparency"),
	Resolution(ResolutionMediaFeature): atom!("resolution"),
	Scan(ScanMediaFeature): atom!("scan"),
	Scripting(ScriptingMediaFeature): atom!("scripting"),
	Update(UpdateMediaFeature): atom!("update"),
	// VerticalViewportSegments(VerticalViewportSegmentsMediaFeature): atom!("vertical-viewport-segments"),
	VideoColorGamut(VideoColorGamutMediaFeature): atom!("video-color-gamut"),
	VideoDynamicRange(VideoDynamicRangeMediaFeature): atom!("video-dynamic-range"),
	Width(WidthMediaFeature): atom!("width"),
);

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub enum MediaType {
	All,          // atom!("all")
	Print,        // atom!("print")
	Screen,       // atom!("screen")
	Custom(Atom), // atom!("tty")
}

impl FromToken for MediaType {
	fn from_token(token: Token) -> Option<Self> {
		match token {
			Token::Ident(ident) => match ident.to_ascii_lowercase() {
				atom!("all") => Some(Self::All),
				atom!("print") => Some(Self::Print),
				atom!("screen") => Some(Self::Screen),
				// https://drafts.csswg.org/mediaqueries/#mq-syntax
				// The <media-type> production does not include the keywords only, not, and, or, and layer.
				atom!("only") | atom!("not") | atom!("and") | atom!("or") | atom!("layer") => None,
				_ => Some(Self::Custom(ident)),
			},
			_ => None,
		}
	}
}

impl<'a> WriteCss<'a> for MediaType {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::All => atom!("all").write_css(sink),
			Self::Print => atom!("print").write_css(sink),
			Self::Screen => atom!("screen").write_css(sink),
			Self::Custom(atom) => atom.write_css(sink),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(MediaRule, 128);
		assert_size!(MediaQueryList, 80);
		assert_size!(MediaQuery, 56);
		assert_size!(MediaCondition, 32);
		assert_size!(MediaType, 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(MediaQuery, "print");
		assert_parse!(MediaQuery, "not embossed");
		assert_parse!(MediaQuery, "only screen");
		assert_parse!(MediaFeature, "(grid)");
		assert_parse!(MediaCondition, "and (grid)");
		assert_parse!(MediaQuery, "screen and (grid)");
		assert_parse!(MediaQuery, "screen and (hover) and (pointer)");
		// assert_parse!(MediaQuery, "screen and (orientation: landscape)");
		assert_parse!(MediaRule, "@media print {\n\n}");
		assert_parse!(MediaRule, "@media print, (prefers-reduced-motion: reduce) {\n\n}");
		// assert_parse!(MediaRule, "@media (min-width: 1200px) {\n\n}");
		// assert_parse!(MediaRUle, "@media only screen and (max-device-width: 800px), only screen and (device-width: 1024px) and (device-height: 600px), only screen and (width: 1280px) and (orientation: landscape), only screen and (device-width: 800px), only screen and (max-width: 767px)");
	}

	#[test]
	fn test_minify() {
		// Drop redundant rules
		assert_minify!(MediaRule, "@media print {}", "");
	}
}