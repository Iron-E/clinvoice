use super::BincodePerson;
use crate::util;
use clinvoice_adapter::{data::{MatchWhen, PersonAdapter, Updatable}, Store};
use clinvoice_data::{Contact, Person, Id};
use std::{collections::HashSet, error::Error, fs, io::BufReader};

impl<'pass, 'path, 'user> PersonAdapter<'pass, 'path, 'user> for BincodePerson<'pass, 'path, 'user>
{
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
	fn create<'name>(
		contact_info: HashSet<Contact>,
		name: &'name str,
		store: Store<'pass, 'path, 'user>,
	) -> Result<Self, Box<dyn Error>>
	{
		Self::init(&store)?;

		let bincode_person = Self
		{
			person: Person
			{
				contact_info,
				id: util::unique_id(&Self::path(&store))?,
				name: name.into(),
			},
			store,
		};

		bincode_person.update()?;

		return Ok(bincode_person);
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store<'pass, 'path, 'user>) -> Result<(), Box<dyn Error>>
	{
		util::create_store_dir(&Self::path(store))?;
		return Ok(());
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
	fn retrieve(
		contact_info: MatchWhen<Contact>,
		id: MatchWhen<Id>,
		name: MatchWhen<String>,
		store: Store<'pass, 'path, 'user>,
	) -> Result<HashSet<Self>, Box<dyn Error>>
	{
		let mut results = HashSet::new();

		for node_path in fs::read_dir(BincodePerson::path(&store))?.filter_map(
			|node| match node {Ok(n) => Some(n.path()), Err(_) => None}
		)
		{
			let person: Person = bincode::deserialize_from(
				BufReader::new(fs::File::open(node_path)?
			))?;

			if contact_info.set_matches(&person.contact_info) &&
				id.is_match(&person.id) &&
				name.is_match(&person.name)
			{
				results.insert(BincodePerson {person, store});
			}
		}

		return Ok(results);
	}
}

#[cfg(test)]
mod tests
{
	use super::{BincodePerson, Contact, HashSet, PersonAdapter, util, Id};
	use std::{fs, io, time::Instant};
	use bincode;

	#[test]
	fn test_create() -> Result<(), io::Error>
	{
		fn assertion(bincode_person: BincodePerson<'_, '_, '_>)
		{
			let start = Instant::now();

			let read_result = fs::read(bincode_person.filepath()).unwrap();
			assert_eq!(*bincode_person, bincode::deserialize(&read_result).unwrap());

			println!("\t----- BincodePerson test_create (read+deserialized file) {}us -----", Instant::now().duration_since(start).as_micros());
		}

		let start = Instant::now();

		return util::test_temp_store(|store|
		{
			let mut contact_info = HashSet::new();

			contact_info.insert(Contact::Address(Id::new_v4()));
			assertion(BincodePerson::create(contact_info.clone(), "", *store).unwrap());

			contact_info.insert(Contact::Email("foo@bar.io".into()));
			assertion(BincodePerson::create(contact_info.clone(), "", *store).unwrap());

			contact_info.insert(Contact::Phone("1-800-555-3600".into()));
			assertion(BincodePerson::create(contact_info.clone(), "", *store).unwrap());

			contact_info.insert(Contact::Address(Id::new_v4()));
			assertion(BincodePerson::create(contact_info.clone(), "", *store).unwrap());

			contact_info.insert(Contact::Email("obviousemail@server.com".into()));
			assertion(BincodePerson::create(contact_info, "", *store).unwrap());

			assert!(fs::remove_dir_all(BincodePerson::path(&store)).is_ok());

			println!("\n>>>>> BincodePerson test_create {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}

	#[test]
	fn test_init() -> Result<(), io::Error>
	{
		let start = Instant::now();

		return util::test_temp_store(|store|
		{
			// Assert that the function can initialize the store.
			assert!(BincodePerson::init(store).is_ok());

			// Create filepath for temporary test file.
			let filepath = BincodePerson::path(store).join("testfile.txt");

			// Assert that creation of a file inside the initialized space is done
			assert!(fs::write(&filepath, "").is_ok());

			// Assert that the function will still return OK with files in the directory.
			assert!(BincodePerson::init(store).is_ok());

			// Assert cleanup
			assert!(fs::remove_file(filepath).is_ok());

			println!("\n>>>>> BincodePerson test_init {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}
}
