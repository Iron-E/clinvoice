#[macro_export]
macro_rules! AdaptJob
{
	($name: ident, $($store_life: lifetime)*) =>
	{
		use
		{
			clinvoice_adapter::Store,
			clinvoice_data::Job,
		};

		/// # Summary
		///
		/// A wrapper around [`Job`] with a [`Store`] that points to its location.
		#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
		pub struct $name<$($store_life),*>
		{
			pub job: Job,
			pub store: Store<$($store_life),*>,
		}

		impl<$($store_life),*> Into<Job> for $name<$($store_life),*>
		{
			fn into(self) -> Job
			{
				return self.job;
			}
		}

		impl<$($store_life),*> Into<Store<$($store_life),*>> for $name<$($store_life),*>
		{
			fn into(self) -> Store<$($store_life),*>
			{
				return self.store;
			}
		}
	};
}
