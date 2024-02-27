use hdx_atom::Atom;
use hdx_lexer::Token;
use miette::{self, Diagnostic};
use thiserror::{self, Error};

use crate::span::Span;

#[derive(Debug, Error, Diagnostic)]
#[error("The token at {0} cannot yet be parsed by the parser :(")]
#[diagnostic(
	help("This feature needs to be implemented within hdx. This file won't parse without it."),
	code(hdx_parser::Unimplemented)
)]
pub struct Unimplemented(#[label("Didn't recognise this bit")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This at-rule mut not have a 'prelude'.")]
#[diagnostic(
	help("The 'prelude' is the bit between the @keyword and the {{"),
	code(hdx_parser::DisllowedAtRulePrelude)
)]
pub struct DisallowedAtRulePrelude(#[label("Remove this part")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This at-rule must not have a 'block'.")]
#[diagnostic(
	help("The 'block' is the bit between the {{ and }}"),
	code(hdx_parser::DisllowedAtRuleBlock)
)]
pub struct DisallowedAtRuleBlock(#[label("Remove this part")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This at-rule must have a 'block'.")]
#[diagnostic(
	help("The 'block' is the bit between the {{ and }}"),
	code(hdx_parser::MissingAtRuleBlock)
)]
pub struct MissingAtRuleBlock(#[label("Add {{}} here")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This declaration wasn't understood, and so was disregarded.")]
#[diagnostic(
	help("The declaration contains invalid syntax, and will be ignored."),
	code(hdx_parser::BadDeclaration)
)]
pub struct BadDeclaration(#[label("This is not valid syntax for a declaration.")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected `{0}`")]
#[diagnostic(help("This is not correct CSS syntax."), code(hdx_parser::Unexpected))]
pub struct Unexpected(pub Token, #[label("This wasn't expected here")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected charset '{0}'. '{0}' isn't allowed here. This must be a valid IANA language code.")]
#[diagnostic(help("Consider removing the rule or setting this to 'utf-8'"), code(hdx_parser::UnexpectedCharset))]
pub struct UnexpectedCharset(
	pub Atom,
	#[label("This charset code is not allowed here")] pub Span,
);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected identifier '{0}'")]
#[diagnostic(help("Try removing the word here."), code(hdx_parser::UnexpectedIdent))]
pub struct UnexpectedIdent(pub Atom, #[label("??")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected identifier '{0}'. '{0}' isn't allowed here, but '{1}' is.")]
#[diagnostic(help("Try changing this to '{1}'"), code(hdx_parser::UnexpectedIdentSuggest))]
pub struct UnexpectedIdentSuggest(
	pub Atom,
	pub Atom,
	#[label("This keyword is not allowed here")] pub Span,
);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected duplicate '{0}'")]
#[diagnostic(help("Try removing the word here."), code(hdx_parser::UnexpectedDuplicateIdent))]
pub struct UnexpectedDuplicateIdent(pub Atom, #[label("Remove this duplicate")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected delimeter '{0}'")]
#[diagnostic(help("Try removing the the character."), code(hdx_parser::UnexpectedDelim))]
pub struct UnexpectedDelim(pub char, #[label("This character wasn't understood")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected pseudo selector ':{0}'")]
#[diagnostic(
	help("This isn't a valid psuedo selector for this rule."),
	code(hdx_parser::UnexpectedPseudo)
)]
pub struct UnexpectedPseudo(pub Atom, #[label("This psuedo selector")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("The dimension '{0}' wasn't recognised for this value type")]
#[diagnostic(
	help(
		"This isn't a recognisable dimension for this value type. If it's a valid dimension, it might be that it cannot be used for this rule or in this position."
	),
	code(hdx_parser::UnexpectedDimension)
)]
pub struct UnexpectedDimension(pub Atom, #[label("This isn't recognised")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected at rule ':{0}'")]
#[diagnostic(
	help(
		"This isn't a recognisable at-rule here. If the rule is valid, it might not be allowed here."
	),
	code(hdx_parser::UnexpectedAtRule)
)]
pub struct UnexpectedAtRule(pub Atom, #[label("This isn't recognised")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected function '{0}'()")]
#[diagnostic(
	help("This function wasn't expected in this position."),
	code(hdx_parser::UnexpectedFunction)
)]
pub struct UnexpectedFunction(pub Atom, #[label("??")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unknown Rule")]
#[diagnostic(
	help("This might be a mistake in the parser, please file an issue!"),
	code(hdx_parser::UnknownRule)
)]
pub struct UnknownRule(#[label("Don't know how to interpret this")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Ignored property due to parse error.")]
#[diagnostic(
	help(
		"This property is going to be ignored because it doesn't look valid. If it is valid, please file an issue!"
	),
	code(hdx_parser::UnknownDeclaration)
)]
pub struct UnknownDeclaration(#[label("This property was ignored.")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unknown Value")]
#[diagnostic(
	help("This might be a mistake in the parser, please file an issue!"),
	code(hdx_parser::UnknownValue)
)]
pub struct UnknownValue(#[label("Don't know how to interpret this")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unknown named color '{0}'")]
#[diagnostic(
	help("Replace this unknown color with a known named color or a valid color value."),
	code(hdx_parser::UnknownColor)
)]
pub struct UnknownColor(pub Atom, #[label("This isn't a known color")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected this to be the end of the file, but there was more content.")]
#[diagnostic(
	help("This is likely a problem with the parser. Please submit a bug report!"),
	code(hdx_parser::ExpectedEnd)
)]
pub struct ExpectedEnd(#[label("All of this extra content was ignored.")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected `{0}` but found `{1}` {2}")]
#[diagnostic(help("This is not correct CSS syntax."), code(hdx_parser::ExpectedToken))]
pub struct ExpectedToken(pub Token, pub Token, #[label("`{0}` expected")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected a dimension but found `{1}`")]
#[diagnostic(help("This is not correct CSS syntax."), code(hdx_parser::ExpectedDimension))]
pub struct ExpectedDimension(pub Token, #[label("dimension expected")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected an identifier but found `{0}`")]
#[diagnostic(help("This is not correct CSS syntax."), code(hdx_parser::ExpectedIdent))]
pub struct ExpectedIdent(pub Token, #[label("This should be `{0}`")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected an identifier but not `{0}`")]
#[diagnostic(help("This is wrong. Maybe it is misspelled?"), code(hdx_parser::ExpectedOtherIdent))]
pub struct ExpectedOtherIdent(pub Atom, #[label("This cannot be `{0}`")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected the identifier `{0}` but found `{1}`")]
#[diagnostic(help("Try changing `{1}` to `{0}`."), code(hdx_parser::ExpectedIdentOf))]
pub struct ExpectedIdentOf(pub Atom, pub Atom, #[label("This should be `{0}`")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected a function but found `{0}`")]
#[diagnostic(help("This is not correct CSS syntax."), code(hdx_parser::ExpectedFunction))]
pub struct ExpectedFunction(pub Token, #[label("This token")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected to see {0}() but saw {1}()")]
#[diagnostic(help("Try changing the {1}() to {0}()"), code(hdx_parser::ExpectedFunctionOf))]
pub struct ExpectedFunctionOf(pub Atom, pub Atom, #[label("This function")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected an @ keyword but saw `{0}`")]
#[diagnostic(help("This is not correct CSS syntax."), code(hdx_parser::ExpectedAtKeyword))]
pub struct ExpectedAtKeyword(pub Token, #[label("This at-keyword")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected to see @{0} but saw @{1}")]
#[diagnostic(help("Try changing the @{1} to @{0}"), code(hdx_parser::ExpectedAtKeywordOf))]
pub struct ExpectedAtKeywordOf(pub Atom, pub Atom, #[label("This at-keyword")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected a delimiter but saw `{0}`")]
#[diagnostic(help("This is not correct CSS syntax."), code(hdx_parser::ExpectedDelim))]
pub struct ExpectedDelim(pub Token, #[label("This at-keyword")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected to see {0} but saw {1}")]
#[diagnostic(help("Try changing the {1} to {0}"), code(hdx_parser::ExpectedDelimOf))]
pub struct ExpectedDelimOf(pub char, pub char, #[label("This delimiter")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected trailing `{0}`")]
#[diagnostic(
	help("Try removing the trailing {0} which will remove this warning."),
	code(hdx_parser::WarnTrailing)
)]
pub struct WarnTrailing(pub Token, #[label("This can be removed")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Invalid hexidecimal value for color: '{0}'")]
#[diagnostic(help("Hex colours must be 3, 4, 6 or 8 digits long."), code(hdx_parser::BadHexColor))]
pub struct BadHexColor(pub Atom, #[label("This is the wrong format")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This block uses an invalid selector, so the whole block would be discarded.")]
#[diagnostic(help("Try adding a selector to this style rule"), code(hdx_parser::NoSelector))]
pub struct NoSelector(#[label("This selector isn't valid")] pub Span, pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This selector has two combinators next to each other, which is disallowed.")]
#[diagnostic(
	help("Try removing one of the combinators or add a selector in between them"),
	code(hdx_parser::AdjacentSelectorCombinators)
)]
pub struct AdjacentSelectorCombinators(
	#[label("...because this combinator is right next to the previous one")] pub Span,
	#[label("This selector is invalid...")] pub Span,
);

#[derive(Debug, Error, Diagnostic)]
#[error("This selector has two types next to each other, which is disallowed.")]
#[diagnostic(
	help("Try removing one of the types or add a space inbetween"),
	code(hdx_parser::AdjacentSelectorTypes)
)]
pub struct AdjacentSelectorTypes(
	#[label("...because this type is right next to the previous one.")] pub Span,
	#[label("This selector is invalid...")] pub Span,
);

#[derive(Debug, Error, Diagnostic)]
#[error("This value isn't allowed to be a raw number, it has to have a dimension.")]
#[diagnostic(
	help("Try adding a dimension, like '{0}'"),
	code(hdx_parser::DisallowedValueWithoutDimension)
)]
pub struct DisallowedValueWithoutDimension(pub Atom, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("A math function isn't valid here.")]
#[diagnostic(
	help("var() and env() can be used but math functions like {0}() cannot."),
	code(hdx_parser::DisallowedMathFunction)
)]
pub struct DisallowedMathFunction(pub Atom, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected a number but saw `{0}`")]
#[diagnostic(help("This is not correct CSS syntax."), code(hdx_parser::ExpectedNumber))]
pub struct ExpectedNumber(pub Token, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected a signed number but saw `{0}`")]
#[diagnostic(help("This number needs a + or a -."), code(hdx_parser::ExpectedSign))]
pub struct ExpectedSign(pub f32, #[label("Add a + here")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected an unsigned number but saw `{0}`")]
#[diagnostic(help("This number cannot have a + or a -."), code(hdx_parser::ExpectedUnsigned))]
pub struct ExpectedUnsigned(pub f32, #[label("Remove the sign")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This number is out of bounds.")]
#[diagnostic(
	help("This needs to be a number between {0} and {1} inclusive."),
	code(hdx_parser::NumberOutOfBounds)
)]
pub struct NumberOutOfBounds(pub f32, pub f32, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This number cannot be negative.")]
#[diagnostic(help("This needs to be greater or equal to 0"), code(hdx_parser::NumberNotNegative))]
pub struct NumberNotNegative(pub f32, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This number is too small.")]
#[diagnostic(help("This needs to be larger than {0}"), code(hdx_parser::NumberTooSmall))]
pub struct NumberTooSmall(pub f32, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This value isn't allowed to have a fraction, it must be a whole number (integer).")]
#[diagnostic(help("Try using {0} instead"), code(hdx_parser::ExpectedInt))]
pub struct ExpectedInt(pub f32, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This value must have a fraction, it must be float.")]
#[diagnostic(help("Try using {0} instead"), code(hdx_parser::ExpectedFloat))]
pub struct ExpectedFloat(pub f32, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This number must be 0, got {0} instead.")]
#[diagnostic(help("Try replacing it with the literal 0 instead"), code(hdx_parser::ExpectedZero))]
pub struct ExpectedZero(pub f32, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Display 'list-item' can only be combined with 'flow' or 'flow-root'")]
#[diagnostic(help("{0} is not valid in combination with list-item, try changing it to flow or flow-root"), code(hdx_parser::DisplayHasInvalidListItemCombo))]
pub struct DisplayHasInvalidListItemCombo(pub Atom, pub Span);
