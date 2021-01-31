use super::BincodeLocation;
use crate::util;
use clinvoice_adapter::{data::{MatchWhen, LocationAdapter, Updatable}, Store};
use clinvoice_data::{Location, Id};
use std::{collections::HashSet, error::Error, fs, io::BufReader};

impl<'pass, 'path, 'user> LocationAdapter<'pass, 'path, 'user> for BincodeLocation<'pass, 'path, 'user>
{
	/// # Summary
	///
	/// Create a new `Location` with a generated ID.
	///
	/// # Parameters
	///
	/// See [`Location`].
	///
	/// # Returns
	///
	/// ```ignore
	/// Location {name, id: /* generated */};
	/// ```
	fn create<'name>(name: &'name str, store: Store<'pass, 'path, 'user>) -> Result<Self, Box<dyn Error>>
	{
		Self::init(&store)?;

		let bincode_person = Self
		{
			location: Location
			{
				id: util::unique_id(&Self::path(&store))?,
				name: name.into(),
				outer_id: None,
			},
			store,
		};

		bincode_person.update()?;

		return Ok(bincode_person);
	}

	/// # Summary
	///
	/// Create a new [`Location`] which is inside of `self`.
	///
	/// # Parameters
	///
	/// See [`Location`].
	///
	/// # Returns
	///
	/// ```ignore
	/// Location {name, id: /* generated */, outside_id: self.unroll().id};
	/// ```
	fn create_inner<'name>(&self, name: &'name str) -> Result<Self, Box<dyn Error>>
	{
		let inner_person = Self
		{
			location: Location
			{
				id: util::unique_id(&Self::path(&self.store))?,
				name: name.into(),
				outer_id: Some(self.id),
			},
			store: self.store,
		};

		inner_person.update()?;

		return Ok(inner_person);
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
	/// Retrieve a [`Location`] from an active [`Store`](core::Store).
	///
	/// # Parameters
	///
	/// See [`Location`].
	///
	/// # Returns
	///
	/// * An [`Error`], when something goes wrong.
	/// * A list of matches, if there are any.
	fn retrieve(
		id: MatchWhen<Id>,
		name: MatchWhen<String>,
		outer: MatchWhen<Option<Id>>,
		store: Store<'pass, 'path, 'user>,
	) -> Result<HashSet<Self>, Box<dyn Error>>
	{
		let mut results = HashSet::new();

		for node_path in fs::read_dir(BincodeLocation::path(&store))?.filter_map(
			|node| match node {Ok(n) => Some(n.path()), Err(_) => None}
		)
		{
			let location: Location = bincode::deserialize_from(
				BufReader::new(fs::File::open(node_path)?
			))?;

			if id.is_match(&location.id) &&
				name.is_match(&location.name) &&
				outer.is_match(&location.outer_id)
			{
				results.insert(BincodeLocation {location, store});
			}
		}

		return Ok(results);
	}
}

#[cfg(test)]
mod tests
{
	use super::{BincodeLocation, LocationAdapter, util};
	use std::{fs, io, time::Instant};

	#[test]
	fn test_create() -> Result<(), io::Error>
	{
		fn assertion(bincode_location: &BincodeLocation<'_, '_, '_>)
		{
			let start = Instant::now();

			let read_result = fs::read(bincode_location.filepath()).unwrap();
			assert_eq!(**bincode_location, bincode::deserialize(&read_result).unwrap());

			println!("\t----- BincodeLocation test_create (read+deserialized file) {}us -----", Instant::now().duration_since(start).as_micros());
		}

		let start = Instant::now();

		return util::test_temp_store(|store|
		{
			let earth = BincodeLocation::create("Earth", *store).unwrap();
			assertion(&earth);

			let usa = earth.create_inner("USA").unwrap();
			assert_eq!(usa.outer_id, Some(earth.id));
			assertion(&usa);

			let arizona = usa.create_inner("Arizona").unwrap();
			assert_eq!(arizona.outer_id, Some(usa.id));
			assertion(&arizona);

			let phoenix = arizona.create_inner("Phoenix").unwrap();
			assert_eq!(phoenix.outer_id, Some(arizona.id));
			assertion(&phoenix);

			assert!(fs::remove_dir_all(BincodeLocation::path(&store)).is_ok());

			println!("\n>>>>> BincodeLocation test_start {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}

	#[test]
	fn test_init() -> Result<(), io::Error>
	{
		let start = Instant::now();

		return util::test_temp_store(|store|
		{
			// Assert that the function can initialize the store.
			assert!(BincodeLocation::init(store).is_ok());

			// Create filepath for temporary test file.
			let filepath = BincodeLocation::path(store).join("testfile.txt");

			// Assert that creation of a file inside the initialized space is done
			assert!(fs::write(&filepath, "").is_ok());

			// Assert that the function will still return OK with files in the directory.
			assert!(BincodeLocation::init(store).is_ok());

			// Assert cleanup
			assert!(fs::remove_file(filepath).is_ok());

			println!("\n>>>>> BincodeLocation test_init {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}
}
