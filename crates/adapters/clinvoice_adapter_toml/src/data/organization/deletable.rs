use super::TomlOrganization;
use clinvoice_adapter::{data::Deletable, Store};
use clinvoice_data::Id;
use std::error::Error;

impl<'pass, 'path, 'user> Deletable<'pass, 'path, 'user>
for TomlOrganization<'_, 'pass, 'path, 'user>
{
	fn delete(store: Store<'pass, 'path, 'user>, id: Id, cascade: bool) -> Result<(), Box<dyn Error>>
	{
		todo!()
	}
}
