use
{
	super::Location,
	crate::data::MatchWhen,
	clinvoice_data::{Id, views::OrganizationView},
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// An [`Organization`](clinvoice_data::Organization) with [matchable](MatchWhen) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Organization<'m>
{
	#[cfg_attr(feature="serde_support", serde(default))]
	pub id: MatchWhen<'m, Id>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub location: Location<'m>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub name: MatchWhen<'m, String>,
}

impl Organization<'_>
{
	/// # Summary
	///
	/// Return `true` if `organization` is a match.
	pub fn any_matches_view(&self, organizations: &[&OrganizationView]) -> bool
	{
		organizations.iter().any(|o| self.matches_view(o))
	}

	/// # Summary
	///
	/// Return `true` if `organization` is a match.
	pub fn matches(&self, organization: &clinvoice_data::Organization) -> bool
	{
		self.id.matches(&organization.id) &&
		self.location.id.matches(&organization.location_id) &&
		self.name.matches(&organization.name)
	}

	/// # Summary
	///
	/// Return `true` if `organization` is a match.
	pub fn matches_view(&self, organization: &OrganizationView) -> bool
	{
		self.id.matches(&organization.id) &&
		self.location.matches_view(&organization.location) &&
		self.name.matches(&organization.name)
	}
}
