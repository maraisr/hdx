use hdx_ast::css::values::{backgrounds::*, Shorthand};

use crate::{CssWriter, Result, WriteCss};

impl<'a> WriteCss<'a> for LineWidth {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> Result {
		todo!()
	}
}

impl<'a> WriteCss<'a> for BorderShorthand<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		if let Shorthand::Explicit(line_width) = &self.line_width {
			line_width.write_css(sink)?;
			if self.line_style.is_explicit() {
				sink.write_char(' ')?;
			}
		}
		if let Shorthand::Explicit(line_style) = &self.line_style {
			line_style.write_css(sink)?;
			if self.color.is_explicit() {
				sink.write_char(' ')?;
			}
		}
		if let Shorthand::Explicit(color) = &self.color {
			color.write_css(sink)?;
		}
		Ok(())
	}
}
