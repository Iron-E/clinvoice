use super::TomlEmployee;
use clinvoice_data::Organization;
use std::error::Error;

impl<'name> Into<Result<Organization<'name>, Box<dyn Error>>> for TomlEmployee<'_, '_, '_, '_, '_, '_, '_>
{
	fn into(self) -> Result<Organization<'name>, Box<dyn Error>>
	{
		todo!()
	}
}