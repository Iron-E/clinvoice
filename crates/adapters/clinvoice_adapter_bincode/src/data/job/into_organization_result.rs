use super::BincodeJob;
use clinvoice_data::Organization;
use std::error::Error;

impl Into<Result<Organization, Box<dyn Error>>> for BincodeJob<'_, '_, '_>
{
	fn into(self) -> Result<Organization, Box<dyn Error>>
	{
		todo!()
	}
}

