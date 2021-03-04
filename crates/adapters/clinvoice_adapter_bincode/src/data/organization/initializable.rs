use
{
	super::BincodeOrganization,
	crate::{data::Result, util},
	clinvoice_adapter::{data::Initializable, Store},
};

impl Initializable for BincodeOrganization<'_, '_, '_>
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

