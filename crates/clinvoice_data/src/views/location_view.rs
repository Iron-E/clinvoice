#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// A view of [`Location`](crate::Location).
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct LocationView
{
	/// # Summary
	///
	/// The [`Location`][location]s which this [`Location`][location] is inside of.
	///
	/// * In order of innermost to outermost.
	///
	/// [location]: crate::Location
	pub outer: Option<Vec<Self>>,

	/// # Summary
	///
	/// The name of the [`Location`].
	pub name: String,
}
