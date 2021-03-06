use std::error::Error;

/// # Summary
///
/// A structure which can be updated on some remote [`Store`][store].
pub trait Updatable
{
	type Error : Error;

	/// # Summary
	///
	/// Send this entity's data to the active [`Store`][store].
	///
	/// # Remarks
	///
	/// This function is called by create methods in order to write a generated entity to some
	/// [`Store`][store]. Manually creating an entity and running this function is not advised, as
	/// it does not guarantee the ID of an entity will be unique.
	///
	/// Rather, it is better to retrieve an entity or create one and then update it.
	///
	/// # Returns
	///
	/// * `()`, on a success.
	/// * An `Error`, when something goes wrong.
	///
	/// [store]: crate::Store
	fn update(&self) -> Result<(), Self::Error>;
}
