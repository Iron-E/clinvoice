mod employees;
mod invoices;
mod store_value;
mod timesheets;

use clinvoice_adapter::Store;
pub use self::{employees::Employees, invoices::Invoices, store_value::StoreValue, timesheets::Timesheets};

use std::collections::HashMap;

/// # Summary
///
/// The `Config` contains settings that affect all areas of the application.
#[derive(Debug)]
pub struct Config<'alias, 'name, 'pass, 'path, 'user>
{
	/// # Summary
	///
	/// Configurations for [`Employee`](clinvoice_data::employee::Employee)s.
	pub employees: Employees,

	/// # Summary
	///
	/// Configurations for [`Invoice`](clinvoice_data::invoice::Invoice)s.
	pub invoices: Invoices,

	/// # Summary
	///
	/// Configurations for data storages.
	stores: HashMap<&'name str, StoreValue<'alias, 'pass, 'path, 'user>>,

	/// # Summary
	///
	/// Configurations for [`Timesheet`](clinvoice_data::timesheet:Timesheet)s.
	pub timesheets: Timesheets,
}

impl Config<'_, '_, '_, '_, '_>
{
	/// # Summary
	///
	/// Get the [`Store`] from `name`, resolving any [`StoreValue::Alias`] which `name` may point to.
	///
	/// # Parameters
	///
	/// * `name`, the name of the [`Store`] which should be returned.
	///
	/// # Returns
	///
	/// The [`Store`] which corresponds to `name`.
	pub fn get_store(&self, name: &str) -> Option<&Store<'_, '_, '_>>
	{
		return match self.stores.get(name)
		{
			None => None,
			Some(value) => match value
			{
				StoreValue::Alias(alias) => self.get_store(alias),
				StoreValue::Storage(store) => Some(store),
			},
		};
	}
}

#[cfg(test)]
mod tests
{
	use clinvoice_data::{chrono::Duration, rusty_money::Iso};
	use clinvoice_adapter::Adapters;
	use super::{Config, Employees, HashMap, Invoices, Store, StoreValue, Timesheets};

	#[test]
	fn test_get_store()
	{
		let mut stores = HashMap::new();

		stores.insert("a", StoreValue::Alias("b"));
		stores.insert("b", StoreValue::Alias("c"));
		stores.insert("c", StoreValue::Storage(Store {
			adapter: Adapters::POSTGRES,
			password: None,
			path: "c/path",
			username: None,
		}));
		stores.insert("d", StoreValue::Storage(Store {
			adapter: Adapters::POSTGRES,
			password: Some("asldkj"),
			path: "d/path",
			username: None,
		}));
		stores.insert("e", StoreValue::Alias("d"));

		let conf = Config
		{
			employees: Employees {default_id: 0},
			invoices: Invoices {default_currency: Iso::USD},
			stores,
			timesheets: Timesheets {interval: Duration::minutes(1)},
		};

		// Reflexivity
		assert_eq!(conf.get_store("a").as_deref(), conf.get_store("b").as_deref());
		assert_eq!(conf.get_store("b").as_deref(), conf.get_store("c").as_deref());
		assert_eq!(conf.get_store("a").as_deref(), conf.get_store("c").as_deref());
		assert_eq!(conf.get_store("d").as_deref(), conf.get_store("e").as_deref());

		// Should never be the same
		assert_ne!(conf.get_store("c").as_deref(), conf.get_store("d").as_deref());
		assert_ne!(conf.get_store("a").as_deref(), conf.get_store("e").as_deref());
	}
}
