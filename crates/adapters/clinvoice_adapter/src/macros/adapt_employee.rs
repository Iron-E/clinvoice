#[macro_export]
macro_rules! AdaptEmployee
{
	($name: ident, $emp_life: lifetime, $store_life: lifetime) =>
	{
		use
		{
			clinvoice_adapter::Store,
			clinvoice_data::Employee,
		};

		/// # Summary
		///
		/// A wrapper around [`Employee`] with a [`Store`] that points to its location.
		#[derive(Clone, Debug, Eq, PartialEq)]
		pub struct $name<$emp_life, $store_life>
		{
			pub employee: &$emp_life Employee,
			pub store: &$store_life Store,
		}
	};
}
