use super::{AnyValue, Deletable, Updatable};
use crate::Store;
use clinvoice_data::{chrono::{DateTime, Utc}, Id, Invoice, Job, Organization, rusty_money::Money, Timesheet};
use std::{collections::BTreeSet, error::Error};

pub trait JobAdapter<'objectives, 'name, 'notes, 'pass, 'path, 'title, 'user, 'work_notes> :
	Deletable<'pass, 'path, 'user> +
	Into<Job<'objectives, 'notes, 'work_notes>> +
	Into<Result<Organization<'name>, Box<dyn Error>>> +
	Into<Store<'pass, 'path, 'user>> +
	Updatable +
{
	/// # Summary
	///
	/// Create a new [`Person`] on the active [`Store`](crate::Store).
	///
	/// # Paramters
	///
	/// See [`Job`].
	///
	/// # Returns
	///
	/// The newly created [`Person`].
	fn create(
		client: Organization<'name>,
		date_open: DateTime<Utc>,
		hourly_rate: Money,
		objectives: &'objectives str,
		store: Store<'pass, 'path, 'user>,
	) -> Result<Self, Box<dyn Error>>;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store<'pass, 'path, 'user>) -> Result<(), Box<dyn Error>>;

	/// # Summary
	///
	/// Retrieve some [`Person`] from the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Job`].
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Job`]s.
	fn retrieve(
		client: AnyValue<Organization<'name>>,
		date_close: AnyValue<DateTime<Utc>>,
		date_open: AnyValue<DateTime<Utc>>,
		id: AnyValue<Id>,
		invoice_date_issued: AnyValue<DateTime<Utc>>,
		invoice_date_paid: AnyValue<DateTime<Utc>>,
		invoice_hourly_rate: AnyValue<Money>,
		objectives: AnyValue<&'objectives str>,
		notes: AnyValue<&'notes str>,
		store: Store<'pass, 'path, 'user>,
	) -> Result<BTreeSet<Self>, Box<dyn Error>>;
}
