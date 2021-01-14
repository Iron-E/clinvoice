use super::{AnyValue, Deletable, Updatable};
use clinvoice_data::{Contact, Id, Person};
use std::error::Error;

pub trait CrudPerson<'contact_info, 'err, 'email, 'name, 'phone> :
	Deletable<'err> +
	From<Person<'contact_info, 'email, 'name, 'phone>> +
	Updatable<'err> +
where
	'email : 'contact_info,
	'phone : 'contact_info,
{
	/// # Summary
	///
	/// Create a new [`Person`] on the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Person`].
	///
	/// # Returns
	///
	/// The newly created [`Person`].
	fn create(contact_info: &'contact_info [Contact<'email, 'phone>], name: &'name str) -> Result<Self, &'err dyn Error>;

	/// # Summary
	///
	/// Retrieve some [`Person`] from the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Person`].
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Job`]s.
	fn retrieve<'arr>(
		contact_info: AnyValue<&'contact_info [Contact<'email, 'phone>]>,
		id: AnyValue<Id>,
		name: AnyValue<&'name str>,
	) -> Result<Option<&'arr [Self]>, &'err dyn Error>;
}
