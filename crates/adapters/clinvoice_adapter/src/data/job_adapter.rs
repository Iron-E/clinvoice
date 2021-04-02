use
{
	super::{Deletable, Initializable, query, Updatable},
	crate::Store,
	clinvoice_data::
	{
		chrono::{DateTime, Utc},
		Job, Money, Organization, views::JobView
	},
	std::error::Error,
};

pub trait JobAdapter<'store> :
	Deletable<Error=<Self as JobAdapter<'store>>::Error> +
	Initializable<Error=<Self as JobAdapter<'store>>::Error> +
	Into<Result<JobView, <Self as JobAdapter<'store>>::Error>> +
	Into<Result<Organization, <Self as JobAdapter<'store>>::Error>> +
	Updatable<Error=<Self as JobAdapter<'store>>::Error> +
{ type Error : Error;

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
		client: Organization,
		date_open: DateTime<Utc>,
		hourly_rate: Money,
		objectives: &str,
		store: &'store Store,
	) -> Result<Job, <Self as JobAdapter<'store>>::Error>;

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
		query: query::Job,
		store: &Store,
	) -> Result<Vec<Job>, <Self as JobAdapter<'store>>::Error>;
}
