use
{
	super::{Invoice, Organization, Timesheet},
	crate::data::Match,

	clinvoice_data::
	{
		chrono::{DateTime, Local},
		Id,
		views::JobView,
	},
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// An [`Job`](clinvoice_data::Job) with [matchable](Match) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Job<'m>
{
	#[cfg_attr(feature="serde_support", serde(default))]
	pub client: Organization<'m>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub date_close: Match<'m, Option<DateTime<Local>>>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub date_open: Match<'m, DateTime<Local>>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub id: Match<'m, Id>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub invoice: Invoice<'m>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub notes: Match<'m, String>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub objectives: Match<'m, String>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub timesheets: Timesheet<'m>,
}

impl Job<'_>
{
	/// # Summary
	///
	/// Return `true` if `job` is a match.
	pub fn matches(&self, job: &clinvoice_data::Job) -> bool
	{
		self.client.id.matches(&job.client_id) &&
		self.date_close.matches(&job.date_close.map(|date| DateTime::from(date))) &&
		self.date_open.matches(&DateTime::from(job.date_open)) &&
		self.id.matches(&job.id) &&
		self.invoice.matches(&job.invoice) &&
		self.notes.matches(&job.notes) &&
		self.objectives.matches(&job.objectives) &&
		self.timesheets.set_matches(job.timesheets.as_slice())
	}

	/// # Summary
	///
	/// Return `true` if `job` is a match.
	pub fn matches_view(&self, job: &JobView) -> bool
	{
		self.client.matches_view(&job.client) &&
		self.date_close.matches(&job.date_close.map(|date| DateTime::from(date))) &&
		self.date_open.matches(&DateTime::from(job.date_open)) &&
		self.id.matches(&job.id) &&
		self.invoice.matches(&job.invoice) &&
		self.notes.matches(&job.notes) &&
		self.objectives.matches(&job.objectives) &&
		self.timesheets.any_matches_view(job.timesheets.as_slice())
	}
}
