use super::TomlOrganization;
use clinvoice_adapter::{data::{AnyValue, OrganizationAdapter}, Store};
use clinvoice_data::{Employee, Id, Location};
use std::{collections::HashSet, error::Error};

impl<'contact_info, 'email, 'err, 'name, 'pass, 'path, 'phone, 'title, 'user> OrganizationAdapter<'contact_info, 'email, 'err, 'name, 'pass, 'path, 'phone, 'title, 'user>
for TomlOrganization<'name, 'pass, 'path, 'user>
where
	'email : 'contact_info,
	'phone : 'contact_info,
{
	/// # Summary
	///
	/// Create a new [`Organization`] on the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Organization`].
	///
	/// # Returns
	///
	/// The newly created [`Organization`].
	fn create(
		location: Location<'name>,
		name: &'name str,
		representatives: HashSet<Employee>,
		store: Store<'pass, 'path, 'user>,
	) -> Result<Self, &'err dyn Error>
	{
		todo!()
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: Store<'pass, 'path, 'user>) -> Result<(), &'err dyn Error>
	{
		todo!()
	}

	/// # Summary
	///
	/// Retrieve some [`Organization`] from the active [`Store`]crate::Store).
	///
	/// # Parameters
	///
	/// See [`Organization`].
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Job`]s.
	fn retrieve<'arr>(
		id: AnyValue<Id>,
		location: AnyValue<Location<'name>>,
		name: AnyValue<&'name str>,
		representatives: AnyValue<HashSet<Employee>>,
		store: Store<'pass, 'path, 'user>,
	) -> Result<Option<&'arr [Self]>, &'err dyn Error>
	{
		todo!()
	}
}

