use
{
	clinvoice_data::Id,

	thiserror::Error,
};

/// # Summary
///
/// Errors for the data
#[derive(Clone, Debug, Error)]
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
	/// An error occurred while querying.
	#[cfg_attr(debug_assertions,      error("{0:?}"))]
	#[cfg_attr(not(debug_assertions), error("{0}"))]
	Query(#[from] clinvoice_query::Error),

	/// # Summary
	///
	/// At least one of some entity is necessary to perform an operation, but none were found.
	#[error("No `{0}` could be retrieved for this operation, which required at least one")]
	NoData(&'static str),
}

clinvoice_error::AliasResult!();
