#[macro_export]
macro_rules! AdaptLocation
{
	($name: ident, $($store_life: lifetime)*) =>
	{
		use
		{
			clinvoice_adapter::Store,
			clinvoice_data::Location,
		};

		/// # Summary
		///
		/// A wrapper around [`Location`] with a [`Store`] that points to its location.
		#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
		pub struct $name<$($store_life),*>
		{
			pub location: Location,
			pub store: Store<$($store_life),*>,
		}

		impl<$($store_life),*> Into<Location> for $name<$($store_life),*>
		{
			fn into(self) -> Location
			{
				self.location
			}
		}

		impl<$($store_life),*> Into<Store<$($store_life),*>> for $name<$($store_life),*>
		{
			fn into(self) -> Store<$($store_life),*>
			{
				self.store
			}
		}
	}
}
