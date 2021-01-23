#[macro_export]
macro_rules! NewtypeEmployee
{
	($name: ident, $($life: lifetime)*, $($store_life: lifetime)*) =>
	{
		use clinvoice_adapter::Store;
		use clinvoice_data::Employee;

		/// # Summary
		///
		/// Wrapper around [`Employee`].
		#[derive(Debug)]
		pub struct $name<$($life),*, $($store_life),*> where
			'email : 'contact_info,
			'phone : 'contact_info,
		{
			pub employee: Employee<$($life),*>,
			pub store: Store<$($store_life),*>,
		}

		impl<$($life),*, $($store_life),*> Into<Employee<$($life),*>> for $name<$($life),*, $($store_life),*> where
			 'email : 'contact_info,
			 'phone : 'contact_info,
		{
			fn into(self) -> Employee<$($life),*>
			{
				return self.employee;
			}
		}

		impl<$($life),*, $($store_life),*> Into<Store<$($store_life),*>> for $name<$($life),*, $($store_life),*> where
			 'email : 'contact_info,
			 'phone : 'contact_info,
		{
			fn into(self) -> Store<$($store_life),*>
			{
				return self.store;
			}
		}
	};
}
