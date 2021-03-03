use
{
	super::BincodeLocation,
	crate::
	{
		data::{Error, Result},
		util,
	},
	clinvoice_adapter::{data::Initializable, Store},
};

impl Initializable<Error> for BincodeLocation<'_, '_, '_>
{
	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store) -> Result<()>
	{
		util::create_store_dir(&Self::path(store))?;
		Ok(())
	}
}

