use super::BincodeOrganization;
use clinvoice_data::Location;
use std::error::Error;

impl<'name> Into<Result<Location<'name>, Box<dyn Error>>> for BincodeOrganization<'name, '_, '_, '_>
{
	fn into(self) -> Result<Location<'name>, Box<dyn Error>>
	{
		todo!()
	}
}

