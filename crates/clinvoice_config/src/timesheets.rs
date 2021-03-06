use
{
	core::time::Duration,

	serde::{Deserialize, Serialize},
};

/// # Summary
///
/// Configurations for [`Timesheet`](clinvoice_data::timesheet:Timesheet)s.
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Timesheets
{
	/// # Summary
	///
	/// The amount of time between increments to the [`crate::toml::Timesheet::time_end`] on a timesheet.
	///
	/// # Example
	///
	/// ```rust
	/// use clinvoice_config::Timesheets;
	/// use std::time::Duration;
	///
	/// // 5 minute interval
	/// Timesheets {interval: Duration::new(300, 0)};
	/// ```
	#[serde(with="humantime_serde")]
	pub interval: Duration,
}
