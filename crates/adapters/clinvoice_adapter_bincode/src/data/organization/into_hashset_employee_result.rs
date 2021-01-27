use super::BincodeOrganization;
use clinvoice_data::Employee;
use std::{collections::HashSet, error::Error};

impl<'email, 'phone, 'title> Into<Result<HashSet<Employee<'email, 'phone, 'title>>, Box<dyn Error>>>
for BincodeOrganization<'_, '_, '_, '_>
{
	fn into(self) -> Result<HashSet<Employee<'email, 'phone, 'title>>, Box<dyn Error>>
	{
		todo!()
	}
}
