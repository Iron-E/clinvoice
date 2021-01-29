use super::BincodeLocation;
use crate::util;
use clinvoice_adapter::{data::{AnyValue, LocationAdapter, Updatable}, Store};
use clinvoice_data::{Location, Id};
use std::{collections::HashSet, error::Error};

impl<'name, 'pass, 'path, 'user> LocationAdapter<'name, 'pass, 'path, 'user>
for BincodeLocation<'name, 'pass, 'path, 'user>
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
	fn create(name: &'name str, store: Store<'pass, 'path, 'user>) -> Result<Self, Box<dyn Error>>
	{
		Self::init(&store)?;

		let bincode_person = Self
		{
			location: Location
			{
				id: util::unique_id(&Self::path(&store))?,
				name,
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
	fn create_inner(&self, name: &'name str) -> Result<Self, Box<dyn Error>>
	{
		let inner_person = Self
		{
			location: Location
			{
				id: util::unique_id(&Self::path(&self.store))?,
				name,
				outer_id: Some(self.location.id),
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
		id: AnyValue<Id>,
		name: AnyValue<&'name str>,
		outer: AnyValue<Location>,
		store: Store<'pass, 'path, 'user>,
	) -> Result<HashSet<Self>, Box<dyn Error>>
	{
		todo!()
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
		fn assertion(bincode_location: BincodeLocation<'_, '_, '_, '_>)
		{
			let read_result = fs::read(bincode_location.filepath()).unwrap();

			assert_eq!(bincode_location.location, bincode::deserialize(&read_result).unwrap());
		}

		let start = Instant::now();

		return util::test_temp_store(|store|
		{
			let earth = BincodeLocation::create("Earth", *store).unwrap();
			assertion(earth);

			let usa = earth.create_inner("USA").unwrap();
			assert_eq!(usa.location.outer_id, Some(earth.location.id));
			assertion(usa);

			let arizona = usa.create_inner("Arizona").unwrap();
			assert_eq!(arizona.location.outer_id, Some(usa.location.id));
			assertion(arizona);

			let phoenix = arizona.create_inner("Phoenix").unwrap();
			assert_eq!(phoenix.location.outer_id, Some(arizona.location.id));
			assertion(phoenix);

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
