use crate::Adapters;

/// # Summary
///
/// A place for CLInvoice to store information.
///
/// # Remarks
///
/// A `Store` can be either on a local or remote filesystem. This filesystem may or may not be a
/// database.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Store<'pass, 'path, 'user>
{
	/// # Summary
	///
	/// The adapter to use for this [`Store`].
	pub adapter: Adapters,

	/// # Summary
	///
	/// The password needed to access the filesystem.
	///
	/// # Remarks
	///
	/// A password may or may not be accompanied by a username. The username is only necessary in a
	/// networked login environment, whereas a password may be used for locally encrypted files.
	pub password: Option<&'pass str>,

	/// # Summary
	///
	/// The place where the data can be found.
	///
	/// # Remarks
	///
	/// * If the store is a local filesystem, this would be a path to the root of the store.
	/// * If the store is a database, this path might be a schema and/or database.
	///
	/// The user shouldn't necessarily have to worry about what this value is, just that it follows
	/// a consistent format which is documented for the adapter.
	pub path: &'path str,

	/// # Summary
	///
	/// The username needed to acces the filesystem.
	///
	/// # Remarks
	///
	/// A username may or may not be accompanied by a password. While commonplace, it is not
	/// mandated that each instance of a user account be protected by a password.
	pub username: Option<&'user str>,
}
