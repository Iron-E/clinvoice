use chrono::{DateTime, TimeZone};

use super::employee::Employee;

/// # Summary
///
/// A `Timesheet` contains all information pertaining to work that has been performed during a
/// specific period of time while working on a [`Job`](super::job::Job)
///
/// # Remarks
///
/// It is likely that a given CLInvoice business object will contain multiple timesheets. As such,
/// it is proposed that the container for business logic contain an array of `Timesheet`, rather
/// than only one.
pub struct Timesheet<'work_notes, Tz> where Tz : TimeZone
{
	/// # Summary
	///
	/// The person who performed this work.
	pub employee: Employee,

	/// # Summary
	///
	/// The time at which this period of work began.
	pub time_begin: DateTime<Tz>,

	/// # Summary
	///
	/// The time at which this period of work ended.
	pub time_end: Option<DateTime<Tz>>,

	/// # Summary
	///
	/// A summary of what work was performed
	///
	/// # Example
	///
	/// > __Note:__ the `str` may contain any valid markdown.
	///
	/// ```markdown
	/// * Researched alternative solutions to image rendering issue.
	/// * Implemented chosen solution.
	/// * Created tests for chosen solution.
	/// ```
	pub work_notes: &'work_notes str,
}
