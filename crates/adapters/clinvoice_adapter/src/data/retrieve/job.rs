use
{
	super::{Invoice, Organization, Timesheet},
	crate::data::MatchWhen,
	clinvoice_data::
	{
		chrono::{DateTime, Utc},
		Id,
		views::JobView,
	},
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// An [`Job`](clinvoice_data::Job) with [matchable](MatchWhen) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Job<'m>
{
	#[cfg_attr(feature="serde_support", serde(default))]
	pub client: Organization<'m>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub date_close: MatchWhen<'m, Option<DateTime<Utc>>>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub date_open: MatchWhen<'m, DateTime<Utc>>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub id: MatchWhen<'m, Id>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub invoice: Invoice<'m>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub notes: MatchWhen<'m, String>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub objectives: MatchWhen<'m, String>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub timesheet: Timesheet<'m>,
}

impl Job<'_>
{
	/// # Summary
	///
	/// Return `true` if `job` is a match.
	pub fn matches(&self, job: &clinvoice_data::Job) -> bool
	{
		todo!()
	}

	/// # Summary
	///
	/// Return `true` if `job` is a match.
	pub fn matches_view(&self, job: &JobView) -> bool
	{
		todo!()
	}
}