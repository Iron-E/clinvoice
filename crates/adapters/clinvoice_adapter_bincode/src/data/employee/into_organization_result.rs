use
{
	crate::data::{BincodeEmployee, BincodeOrganization, Result},
	clinvoice_adapter::data::{Error as DataError, MatchWhen, OrganizationAdapter},
	clinvoice_data::Organization,
};

impl Into<Result<Organization>> for BincodeEmployee<'_, '_, '_>
{
	fn into(self) -> Result<Organization>
	{
		let results = BincodeOrganization::retrieve(
			MatchWhen::EqualTo(self.employee.organization_id), // id
			MatchWhen::Any, // location
			MatchWhen::Any, // name
			self.store,
		)?;

		let bincode_organization = match results.get(0)
		{
			Some(bin_org) => bin_org,
			_ => return Err(DataError::DataIntegrity {id: self.employee.organization_id}.into()),
		};

		Ok(bincode_organization.organization.clone())
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeEmployee, BincodeOrganization, Result, OrganizationAdapter},
		crate::util,
		clinvoice_adapter::data::EmployeeAdapter,
		clinvoice_data::{Contact, EmployeeStatus, Id, Location, Organization, Person},
		std::time::Instant,
	};

	#[test]
	fn test_into_organization()
	{
		util::test_temp_store(|store|
		{
			let dogood = BincodeOrganization::create(
				Location {name: "Earth".into(), id: Id::new_v4(), outer_id: None},
				"DoGood Inc",
				*store
			).unwrap();

			let testy = BincodeEmployee::create(
				vec![Contact::Email("foo".into())],
				dogood.organization.clone(),
				Person
				{
					contact_info: vec![Contact::Email("yum".into())],
					id: Id::new_v4(),
					name: "Testy Mćtesterson".into(),
				},
				"CEO of Tests",
				EmployeeStatus::Employed,
				*store,
			).unwrap();

			let start = Instant::now();
			let testy_org: Result<Organization> = testy.into();
			println!("\n>>>>> BincodeEmployee::into_organization {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

			assert_eq!(dogood.organization, testy_org.unwrap());
		});
	}
}
