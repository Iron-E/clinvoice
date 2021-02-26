use
{
	crate::data::{BincodeEmployee, BincodePerson},
	clinvoice_adapter::
	{
		data::{Error as DataError, MatchWhen, PersonAdapter},
		DynamicResult,
	},
	clinvoice_data::Person,
};

impl Into<DynamicResult<Person>> for BincodeEmployee<'_, '_, '_>
{
	fn into(self) -> DynamicResult<Person>
	{
		let results = BincodePerson::retrieve(
			MatchWhen::Any, // contact into
			MatchWhen::EqualTo(self.employee.person_id), // id
			MatchWhen::Any, // name
			self.store,
		)?;

		let bincode_person = match results.iter().next()
		{
			Some(bin_org) => bin_org,
			_ => return Err(DataError::DataIntegrity {id: self.employee.person_id}.into()),
		};

		Ok(bincode_person.person.clone())
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeEmployee, BincodePerson, DynamicResult, PersonAdapter},
		crate::util,
		clinvoice_adapter::data::EmployeeAdapter,
		clinvoice_data::{Contact, EmployeeStatus, Id, Organization, Person},
		std::time::Instant,
	};

	#[test]
	fn test_into_organization()
	{
		util::test_temp_store(|store|
		{
			let testy = BincodePerson::create(
				vec![Contact::Email("yum".into())],
				"Testy Mćtesterson".into(),
				*store,
			).unwrap();

			let testy_employed = BincodeEmployee::create(
				vec![Contact::Email("foo".into())],
				Organization
				{
					id: Id::new_v4(),
					location_id: Id::new_v4(),
					name: "DoGood Inc".into(),
				},
				testy.person.clone(),
				"CEO of Tests",
				EmployeeStatus::NotEmployed,
				*store,
			).unwrap();

			let start = Instant::now();
			let testy_person: DynamicResult<Person> = testy_employed.into();
			println!("\n>>>>> BincodeEmployee::into_person {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

			assert_eq!(testy.person, testy_person.unwrap());
		});
	}
}
