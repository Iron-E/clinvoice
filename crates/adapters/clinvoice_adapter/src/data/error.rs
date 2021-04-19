use
{
	clinvoice_data::Id,

	thiserror::Error,
};

/// # Summary
///
/// Errors for the data
#[derive(Copy, Clone, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error
{
	/// # Summary
	///
	/// Some reference to an `id` was expected, but none was found.
	#[error("A reference to ID #{0} was expected, but `None` was found")]
	DataIntegrity(Id),

	/// # Summary
	///
	/// Some reference to an `id` was expected, but none was found.
	#[error("Attempted to delete ID #{0}, but one or more other entities require it. Cascade delete to remove them")]
	DeleteRestricted(Id),

	/// # Summary
	///
	/// At least one of some entity is necessary to perform an operation, but none were found.
	#[error("No `{0}` could be retrieved for this operation, which required at least one")]
	NoData(&'static str),
}

clinvoice_error::AliasResult!();
