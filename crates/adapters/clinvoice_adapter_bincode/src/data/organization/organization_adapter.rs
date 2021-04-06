use
{
	std::{fs, io::BufReader},

	super::BincodeOrganization,
	crate::
	{
		data::{Error, Result},
		util,
	},

	clinvoice_adapter::
	{
		data::{Initializable, OrganizationAdapter, query, Updatable},
		Store,
	},
	clinvoice_data::{Location, Organization},
};

impl<'store> OrganizationAdapter<'store> for BincodeOrganization<'_, 'store>
{
	type Error = Error;

	/// # Summary
	///
	/// Create a new [`Organization`] on the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Organization`].
	///
	/// # Returns
	///
	/// The newly created [`Organization`].
	fn create(location: Location, name: &str, store: &'store Store) -> Result<Organization>
	{
		Self::init(&store)?;

		let organization = Organization
		{
			id: util::unique_id(&Self::path(&store))?,
			location_id: location.id,
			name: name.into(),
		};

		BincodeOrganization {organization: &organization, store}.update()?;

		Ok(organization)
	}

	/// # Summary
	///
	/// Retrieve some [`Organization`] from the active [`Store`]crate::Store).
	///
	/// # Parameters
	///
	/// See [`Organization`].
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Job`]s.
	fn retrieve(query: query::Organization, store: &Store) -> Result<Vec<Organization>>
	{
		Self::init(&store)?;

		let mut results = Vec::new();

		for node_path in util::read_files(BincodeOrganization::path(&store))?
		{
			let organization: Organization = bincode::deserialize_from(BufReader::new(
				fs::File::open(node_path)?
			))?;

			if query.matches(&organization)
			{
				results.push(organization);
			}
		}

		Ok(results)
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeOrganization, Location, Organization, OrganizationAdapter, query, Store, util},
		clinvoice_adapter::data::Match,
		clinvoice_data::Id,
		std::{borrow::Cow, fs, time::Instant},
	};

	#[test]
	fn create()
	{
		util::temp_store(|store|
		{
			let earth_id = Id::new_v4();
			let usa_id = Id::new_v4();
			let arizona_id = Id::new_v4();
			let phoenix_id = Id::new_v4();
			let some_id = Id::new_v4();

			let start = Instant::now();

			create_assertion(
				BincodeOrganization::create(
					Location {name: "Earth".into(), id: Id::new_v4(), outer_id: None},
					"alsdkjaldkj", &store
				).unwrap(),
				&store,
			);

			create_assertion(
				BincodeOrganization::create(
					Location {name: "USA".into(), id: usa_id, outer_id: Some(earth_id)},
					"alskdjalgkh  ladhkj EAL ISdh", &store
				).unwrap(),
				&store,
			);

			create_assertion(
				BincodeOrganization::create(
					Location {name: "Arizona".into(), id: arizona_id, outer_id: Some(earth_id)},
					" AAA – 44 %%", &store
				).unwrap(),
				&store,
			);

			create_assertion(
				BincodeOrganization::create(
					Location {name: "Phoenix".into(), id: phoenix_id, outer_id: Some(arizona_id)},
					" ^^^ ADSLKJDLASKJD FOCJCI", &store
				).unwrap(),
				&store,
			);

			create_assertion(
				BincodeOrganization::create(
					Location {name: "Some Road".into(), id: some_id, outer_id: Some(phoenix_id)},
					"aldkj doiciuc giguy &&", &store
				).unwrap(),
				&store,
			);

			println!("\n>>>>> BincodeOrganization::create {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 5);
		});
	}

	fn create_assertion(organization: Organization, store: &Store)
	{
		let read_result = fs::read(BincodeOrganization {organization: &organization, store}.filepath()).unwrap();
		assert_eq!(organization, bincode::deserialize(&read_result).unwrap());
	}

	#[test]
	fn retrieve()
	{
		util::temp_store(|store|
		{
			let earth_id = Id::new_v4();
			let packing = BincodeOrganization::create(
				Location {name: "Earth".into(), id: earth_id, outer_id: None},
				"Packing Co", &store
			).unwrap();

			let usa_id = Id::new_v4();
			let eal = BincodeOrganization::create(
				Location {name: "USA".into(), id: usa_id, outer_id: Some(earth_id)},
				"alskdjalgkh  ladhkj EAL ISdh", &store
			).unwrap();

			let arizona_id = Id::new_v4();
			let aaa = BincodeOrganization::create(
				Location {name: "Arizona".into(), id: arizona_id, outer_id: Some(usa_id)},
				" AAA – 44 %%", &store
			).unwrap();

			let start = Instant::now();
			// retrieve `packing` and `eal`
			let results = BincodeOrganization::retrieve(
				query::Organization
				{
					location: query::Location
					{
						id: Match::HasAny(vec![Cow::Borrowed(&earth_id), Cow::Borrowed(&usa_id)].into_iter().collect()),
						..Default::default()
					},
					name: Match::HasNone(vec![Cow::Borrowed(&aaa.name)].into_iter().collect()),
					..Default::default()
				},
				&store,
			).unwrap();
			println!("\n>>>>> BincodeOrganization::retrieve {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

			// test if `packing` and `eal` were retrieved
			assert!(results.contains(&packing));
			assert!(results.contains(&eal));
			assert!(!results.contains(&aaa));
		});
	}
}
