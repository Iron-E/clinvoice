use clinvoice_data::id::Id;

/// # Summary
///
/// Configurations for [`Employee`](clinvoice_data::employee::Employee)s.
pub struct Employees
{
	/// # Summary
	///
	/// The [`Id`] of the employee which should be defaulted to when attaching to a timesheet.
	pub default_id: Id,
}
