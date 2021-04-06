use
{
	std::{fs, io::BufReader},

	super::BincodePerson,
	crate::
	{
		data::{Error, Result},
		util,
	},

	clinvoice_adapter::
	{
		data::{Initializable, PersonAdapter, query, Updatable},
		Store,
	},
	clinvoice_data::Person,
};

impl<'store> PersonAdapter<'store> for BincodePerson<'_, 'store>
{
	type Error = Error;

	/// # Summary
	///
	/// Create a new [`Person`] on the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Person`].
	///
	/// # Returns
	///
	/// The newly created [`Person`].
	fn create(name: &str, store: &'store Store,) -> Result<Person>
	{
		Self::init(&store)?;

		let person = Person
		{
			id: util::unique_id(&Self::path(&store))?,
			name: name.into(),
		};

		BincodePerson {person: &person, store}.update()?;

		Ok(person)
	}

	/// # Summary
	///
	/// Retrieve some [`Person`] from the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Person`].
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Job`]s.
	fn retrieve(query: query::Person, store: &Store) -> Result<Vec<Person>>
	{
		Self::init(&store)?;

		let mut results = Vec::new();

		for node_path in util::read_files(BincodePerson::path(&store))?
		{
			let person: Person = bincode::deserialize_from(BufReader::new(
				fs::File::open(node_path)?
			))?;

			if query.matches(&person)
			{
				results.push(person);
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
		std::{borrow::Cow, fs, time::Instant},

		super::{BincodePerson, Person, PersonAdapter, query, Store, util},

		clinvoice_adapter::data::Match,
	};

	#[test]
	fn create()
	{
		util::temp_store(|store|
		{
			let start = Instant::now();

			create_assertion(
				BincodePerson::create(
					"Widdle",
					&store,
				).unwrap(),
				&store,
			);

			create_assertion(
				BincodePerson::create(
					"Long",
					&store,
				).unwrap(),
				&store,
			);

			create_assertion(
				BincodePerson::create(
					"Steven",
					&store,
				).unwrap(),
				&store,
			);

			create_assertion(
				BincodePerson::create(
					"JingleBob",
					&store,
				).unwrap(),
				&store,
			);

			create_assertion(
				BincodePerson::create(
					"asldkj jdsoai",
					&store,
				).unwrap(),
				&store,
			);

			println!("\n>>>>> BincodePerson::create {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 5);
		});
	}

	fn create_assertion(person: Person, store: &Store)
	{
		let read_result = fs::read(BincodePerson {person: &person, store}.filepath()).unwrap();
		assert_eq!(person, bincode::deserialize(&read_result).unwrap());
	}

	#[test]
	fn retrieve()
	{
		util::temp_store(|store|
		{
			let flingo = BincodePerson::create(
				"flingo",
				&store
			).unwrap();

			let bob = BincodePerson::create(
				"bob",
				&store
			).unwrap();

			let slimdi = BincodePerson::create(
				"slimdi",
				&store
			).unwrap();

			let longone = BincodePerson::create(
				"longone",
				&store
			).unwrap();

			let start = Instant::now();

			// Retrieve bob
			let only_bob = BincodePerson::retrieve(
				query::Person
				{
					id: Match::EqualTo(Cow::Borrowed(&bob.id)),
					..Default::default()
				},
				&store,
			).unwrap();

			// Retrieve longone and slimdi
			let longone_slimdi = BincodePerson::retrieve(
				query::Person
				{
					name: Match::HasAny(vec![Cow::Borrowed(&slimdi.name.clone()), Cow::Borrowed(&longone.name.clone())].into_iter().collect()),
					..Default::default()
				},
				&store,
			).unwrap();

			println!("\n>>>>> BincodePerson::retrieve {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 2);

			// Assert bob is the only one retrieved
			assert!(!only_bob.contains(&flingo));
			assert!(only_bob.contains(&bob));
			assert!(!only_bob.contains(&slimdi));
			assert!(!only_bob.contains(&longone));

			// Assert bob is the only one retrieved
			assert!(!longone_slimdi.contains(&flingo));
			assert!(!longone_slimdi.contains(&bob));
			assert!(longone_slimdi.contains(&slimdi));
			assert!(longone_slimdi.contains(&longone));
		});
	}
}
