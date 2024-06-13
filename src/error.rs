use proc_macro::Span;

/// Used to report errors.
///
/// When 'pretty_errors' isn't enabled, simply includes a basic message.
/// When enabled, adds a span for the source of the error and a more detailed
/// and helpful message we call hint.
#[derive(Debug)]
pub struct Error
{
	/// Basic error message.
	///
	/// Will always be reported (first).
	msg: String,

	/// The source of the error
	#[cfg(feature = "pretty_errors")]
	span: Span,

	/// Additional error details and help
	#[cfg(feature = "pretty_errors")]
	hint: String,
}

impl Error
{
	/// Creates a basic error.
	pub fn new(msg: impl Into<String>) -> Self
	{
		#[cfg(feature = "pretty_errors")]
		{
			Self {
				msg: msg.into(),
				span: Span::call_site(),
				hint: "".to_string(),
			}
		}
		#[cfg(not(feature = "pretty_errors"))]
		{
			Self { msg: msg.into() }
		}
	}

	/// Adds a span to the error and returns it.
	///
	/// If `pretty_errors` is disabled, does nothing.
	#[allow(unused_variables)]
	#[allow(unused_mut)]
	pub fn span(mut self, span: Span) -> Self
	{
		#[cfg(feature = "pretty_errors")]
		{
			self.span = span;
		}
		self
	}

	/// Adds a hint to the error and returns it.
	///
	/// If `pretty_errors` is disabled, does nothing.
	#[allow(unused_variables)]
	#[allow(unused_mut)]
	pub fn hint(mut self, hint: impl Into<String>) -> Self
	{
		#[cfg(feature = "pretty_errors")]
		{
			self.hint = hint.into();
		}
		self
	}

	pub fn get_span(&self) -> Span
	{
		#[cfg(feature = "pretty_errors")]
		{
			self.span
		}
		#[cfg(not(feature = "pretty_errors"))]
		{
			Span::call_site()
		}
	}

	/// Returns the source span of the error and a full message including a
	/// potential hint (if applicable).
	#[cfg(not(feature = "pretty_errors"))]
	pub fn into_panic_message(self) -> String
	{
		self.msg
	}

	#[cfg(feature = "pretty_errors")]
	pub fn into_diagnostic(self) -> proc_macro2_diagnostics::Diagnostic
	{
		use proc_macro2::Span;
		use proc_macro2_diagnostics::{Diagnostic, Level};
		let mut diagnostic = Diagnostic::spanned(Span::from(self.span), Level::Error, self.msg);
		if !self.hint.is_empty()
		{
			diagnostic = diagnostic.help(self.hint);
		}
		diagnostic
	}
}
