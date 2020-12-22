use clinvoice_data::{chrono::TimeZone, Timesheet};

/// # Summary
///
/// A wrapper around [`Timesheet`] for use with MongoDB.
pub struct MongoTimesheet<'work_notes, Tz : TimeZone>
(
	Timesheet<'work_notes, Tz>,
);
