use
{
	crate::data::{BincodeEmployee, BincodePerson, Result},
	clinvoice_adapter::data::{Error as DataError, Match, PersonAdapter, retrieve},
	clinvoice_data::Person,
	std::borrow::Cow,
};

impl Into<Result<Person>> for BincodeEmployee<'_, '_>
{
	fn into(self) -> Result<Person>
	{
		let results = BincodePerson::retrieve(
			retrieve::Person
			{
				id: Match::EqualTo(Cow::Borrowed(&self.employee.person_id)),
				..Default::default()
			},
			self.store,
		)?;

		let person = match results.get(0)
		{
			Some(prsn) => prsn,
			_ => return Err(DataError::DataIntegrity {id: self.employee.person_id}.into()),
		};

		Ok(person.clone())
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeEmployee, BincodePerson, PersonAdapter, Result},
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
				"Testy Mćtesterson".into(),
				&store,
			).unwrap();

			let testy_employed = BincodeEmployee
			{
				employee: &BincodeEmployee::create(
					vec![("Work Email".into(), Contact::Email("foo".into()))].into_iter().collect(),
					Organization
					{
						id: Id::new_v4(),
						location_id: Id::new_v4(),
						name: "DoGood Inc".into(),
					},
					testy.clone(),
					EmployeeStatus::NotEmployed,
					"CEO of Tests",
					&store,
				).unwrap(),
				store,
			};

			let start = Instant::now();
			let testy_person: Result<Person> = testy_employed.into();
			println!("\n>>>>> BincodeEmployee::into_person {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

			assert_eq!(testy, testy_person.unwrap());
		});
	}
}
