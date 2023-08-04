pub mod angle;
pub mod backgrounds;
pub mod r#box;
pub mod color;
pub mod content;
pub mod display;
pub mod expr;
pub mod fonts;
pub mod inline;
pub mod length;
pub mod non_standard;
pub mod page_floats;
pub mod shorthand;
pub mod size_adjust;
pub mod sizing;
pub mod text_decor;
pub mod ui;

use hdx_ast::css::{properties::Todo, values::*};

use crate::{diagnostics, Atomizable, Parse, Parser, Result, Spanned};

macro_rules! parse_for_enums {
    {$( $prop: ident, )+} => {
        $(
            impl<'a> Parse<'a> for $prop {
                fn parse(parser: &mut Parser<'a>) -> Result<Spanned<$prop>> {
                    let span = parser.cur().span;
                    let ident = parser.expect_ident()?;
                    if let Some(val) = $prop::from_atom(ident.clone()) {
                        Ok(val.spanned(span.up_to(&parser.cur().span)))
                    } else {
                        Err(diagnostics::UnexpectedIdent(ident, span))?
                    }
                }
            }
        )+
    }
}

parse_for_enums! {
	AlignmentBaselineValue,
	AutoOrNone,
	BaselineSourceValue,
	BorderCollapseValue,
	BoxSizingValue,
	CaptionSideValue,
	ClearValue,
	DominantBaselineValue,
	EmptyCellsValue,
	FloatReferenceValue,
	InlineSizingValue,
	LineStyle,
	MinIntrinsicSizingValue,
	OverflowKeyword,
	PositionValue,
	TableLayoutValue,
	TextAlignAllValue,
	TextAlignLastValue,
	TextAlignValue,
	TextDecorationSkipInkValue,
	TextDecorationStyleValue,
	VisibilityValue,
}

// TODO:
impl<'a> Parse<'a> for RatioOrAuto {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		Err(diagnostics::Unimplemented(parser.cur().span))?
	}
}

// TODO:
impl<'a> Parse<'a> for TimeOrAuto {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		Err(diagnostics::Unimplemented(parser.cur().span))?
	}
}

// TODO:
impl<'a> Parse<'a> for LineWidth {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		Err(diagnostics::Unimplemented(parser.cur().span))?
	}
}

impl<'a> Parse<'a> for NoNonGlobalValuesAllowed {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		Err(diagnostics::Unexpected(parser.cur().kind, parser.cur().span))?
	}
}

// TODO:
impl<'a> Parse<'a> for Todo {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		Err(diagnostics::Unimplemented(parser.cur().span))?
	}
}