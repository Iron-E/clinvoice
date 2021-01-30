#[macro_export]
macro_rules! AdaptOrganization
{
	($name: ident, $($life: lifetime)*, $($store_life: lifetime)*) =>
	{
		use clinvoice_adapter::Store;
		use clinvoice_data::Organization;
		use core::ops::Deref;

		/// # Summary
		///
		/// A wrapper around [`Organization`] with a [`Store`] that points to its location.
		#[derive(Clone, Debug, Eq, Hash, PartialEq)]
		pub struct $name<$($life),*, $($store_life),*>
		{
			organization: Organization<$($life),*>,
			pub store: Store<$($store_life),*>,
		}

		impl<$($life),*, $($store_life),*> Deref for $name<$($life),*, $($store_life),*>
		{
			type Target = Organization<$($life),*>;

			fn deref(&self) -> &Self::Target
			{
				return &self.organization;
			}
		}

		impl<$($life),*, $($store_life),*> Into<Organization<$($life),*>> for $name<$($life),*, $($store_life),*>
		{
			fn into(self) -> Organization<$($life),*>
			{
				return self.organization;
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
