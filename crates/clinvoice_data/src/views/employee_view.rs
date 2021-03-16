mod display;
mod hash;
mod partial_eq;
mod preservable_serde;

use
{
	crate::{EmployeeStatus, Id},
	super::{ContactView, OrganizationView, PersonView},
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// A view of [`Employee`](crate::Employee).
#[derive(Clone, Debug, Eq, Ord, PartialOrd)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct EmployeeView
{
	/// # Summary
	///
	/// Contact information specific to the [`Organization`] that the [`Employee`] does work for.
	pub contact_info: Vec<ContactView>,

	/// # Summary
	///
	/// The reference number of this [`Employee`], which can be used instead of the compound key
	/// {`organization`, `person_id`}.
	#[cfg_attr(feature="serde_support", serde(skip))]
	pub id: Id,

	/// # Summary
	///
	/// The reference number of the [`Organization`](crate::Organization) which this
	/// [`Employee`] is in reference to.
	pub organization: OrganizationView,

	/// # Summary
	///
	/// The reference number of the [`Person`](super::person::Person) which this
	/// [`Employee`] is in reference to.
	pub person: PersonView,

	/// # Summary
	///
	/// The status of the employee.
	///
	/// # Remarks
	///
	/// Flagging this field as [`NotEmployed`](EmployeeStatus::NOT_EMPLOYED) is a viable alternative to deletion.
	pub status: EmployeeStatus,

	/// # Summary
	///
	/// The [`Employee`]'s title  in the company.
	///
	/// # Example
	///
	/// * CEO
	/// * Media Manager
	pub title: String,
}
