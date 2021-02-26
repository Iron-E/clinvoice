#[macro_export]
macro_rules! AdaptEmployee
{
	($name: ident, $($store_life: lifetime)*) =>
	{
		use
		{
			clinvoice_adapter::Store,
			clinvoice_data::Employee,
		};

		/// # Summary
		///
		/// A wrapper around [`Employee`] with a [`Store`] that points to its location.
		#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
		pub struct $name<$($store_life),*>
		{
			pub employee: Employee,
			pub store: Store<$($store_life),*>,
		}

		impl<$($store_life),*> Into<Employee> for $name<$($store_life),*>
		{
			fn into(self) -> Employee
			{
				self.employee
			}
		}

		impl<$($store_life),*> Into<Store<$($store_life),*>> for $name<$($store_life),*>
		{
			fn into(self) -> Store<$($store_life),*>
			{
				self.store
			}
		}
	};
}
