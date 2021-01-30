#[macro_export]
macro_rules! AdaptPerson
{
	($name: ident, $($life: lifetime)*, $($store_life: lifetime)*) =>
	{
		use clinvoice_adapter::Store;
		use clinvoice_data::Person;
		use core::ops::Deref;

		/// # Summary
		///
		/// A wrapper around [`Person`] with a [`Store`] that points to its location.
		#[derive(Clone, Debug, Eq, Hash, PartialEq)]
		pub struct $name<$($life),*, $($store_life),*>
		{
			person: Person<$($life),*>,
			pub store: Store<$($store_life),*>,
		}

		impl<$($life),*, $($store_life),*> Deref for $name<$($life),*, $($store_life),*>
		{
			type Target = Person<$($life),*>;

			fn deref(&self) -> &Self::Target
			{
				return &self.person;
			}
		}

		impl<$($life),*, $($store_life),*> Into<Person<$($life),*>> for $name<$($life),*, $($store_life),*>
		{
			fn into(self) -> Person<$($life),*>
			{
				return self.person;
			}
		}

		impl<$($life),*, $($store_life),*> Into<Store<$($store_life),*>> for $name<$($life),*, $($store_life),*>
		{
			fn into(self) -> Store<$($store_life),*>
			{
				return self.store;
			}
		}
	}
}
