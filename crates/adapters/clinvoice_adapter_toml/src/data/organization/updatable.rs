use super::TomlOrganization;
use clinvoice_adapter::data::Updatable;
use std::error::Error;

impl Updatable for TomlOrganization<'_, '_, '_, '_>
{
	fn update(&self) -> Result<(), Box<dyn Error>>
	{
		todo!()
	}
}
