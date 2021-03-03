use
{
	super::{MatchWhen, Deletable, Initializable, Updatable},
	crate::Store,
	clinvoice_data::{Location, Id, views::LocationView},
	std::error::Error,
};

pub trait LocationAdapter<'pass, 'path, 'user> :
	Clone +
	Deletable<Self::Error> +
	Initializable<Self::Error> +
	Into<Location> +
	Into<Result<LocationView, Self::Error>> +
	Into<Store<'pass, 'path, 'user>> +
	Updatable<Self::Error> +
{
	type Error : Error;

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
	fn create(name: &str, store: Store<'pass, 'path, 'user>) -> Result<Self, Self::Error>;

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
	fn create_inner(&self, name: &str) -> Result<Self, Self::Error>;

	/// # Summary
	///
	/// Get the [`Location`]s which contain this [`Location`].
	fn outer_locations(&self) -> Result<Vec<Location>, super::Error>
	{
		let mut outer_locations = Vec::<Location>::new();

		let location: Location = self.clone().into();
		let store: Store = self.clone().into();

		let mut outer_id = location.outer_id;
		while let Some(id) = outer_id
		{
			if let Ok(results) = Self::retrieve(
				MatchWhen::EqualTo(id), // id
				MatchWhen::Any, // name
				MatchWhen::Any, // outer id
				store,
			)
			{
				if let Some(adapted_location) = results.into_iter().next()
				{
					let loc: Location = adapted_location.into();

					outer_id = loc.outer_id;
					outer_locations.push(loc);
					continue;
				}
			}

			return Err(super::Error::DataIntegrity {id});
		}

		Ok(outer_locations)
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
	) -> Result<Vec<Self>, Self::Error>;
}
