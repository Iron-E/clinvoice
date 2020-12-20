use super::id::Id;

pub mod into_organization;

/// # Summary
///
/// An `Emlpoyer` is an [`Organization`](super::organization::Organization) which has hired some
/// [`Employee`](super::emlpoyee::Emlpoyee)s to do [`Job`](super::job::Job)s for
/// [`Client`](super::client::Client)s.
pub struct Employer
{
	/// # Summary
	///
	/// The reference number of the [`Organization`](super::organization::Organization) which this
	/// [`Emlpoyer`] is in reference to.
	_organization_id: Id,
}
