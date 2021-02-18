use crate::DynamicResult;

/// # Summary
///
/// A structure which can be deleted from a remote [`Store`](crate::Store).
pub trait Deletable
{
	/// # Summary
	///
	/// Delete a [`Person`].
	///
	/// # Paramteters
	///
	/// * `id`, the [`Id`] of the [`Person`] to delete.
	/// * `cascade`, whether or not to delete entries which reference this [`Person`].
	///
	/// # Returns
	///
	/// * `()`, on a success.
	/// * An [`Error`] when:
	///   * `self.id` had not already been `create`d.
	///   * Something goes wrong.
	fn delete(&self, cascade: bool) -> DynamicResult<()>;
}
