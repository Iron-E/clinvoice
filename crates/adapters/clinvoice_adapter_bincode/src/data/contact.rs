use
{
	super::BincodeLocation,
	clinvoice_adapter::
	{
		data::{Error as DataError, LocationAdapter, MatchWhen},
		DynamicResult, Store,
	},
	clinvoice_data::{Contact, views::{ContactView, LocationView}},
};

/// # Summary
///
/// Convert some [`Contact`] into a [`ContactView`].
pub fn into_view(contact: Contact, store: Store) -> DynamicResult<ContactView>
{
	Ok(match contact
	{
		Contact::Address(address) => match BincodeLocation::retrieve(
				MatchWhen::EqualTo(address), // id
				MatchWhen::Any, // outer_id
				MatchWhen::Any, // name
				store,
			)?.into_iter().next()
			{
				Some(result) =>
				{
					let view: DynamicResult<LocationView> = result.into();
					ContactView::Address(view?)
				},
				_ => return Err(DataError::DataIntegrity {id: address}.into()),
			},
		Contact::Email(email) => ContactView::Email(email),
		Contact::Phone(phone) => ContactView::Phone(phone),
	})
}

/// # Summary
///
/// Convert some [`Contact`] into a [`ContactView`].
pub fn into_views<I>(contact_info: I, store: Store) -> DynamicResult<Vec<ContactView>> where I : IntoIterator<Item = Contact>
{
	let contact_info_view_result = contact_info.into_iter().map(|c| into_view(c, store));
	let mut contact_info_view = Vec::new();

	for result in contact_info_view_result
	{
		match result
		{
			Ok(contact) => contact_info_view.push(contact),
			Err(e) => return Err(e),
		};
	}

	Ok(contact_info_view)
}
