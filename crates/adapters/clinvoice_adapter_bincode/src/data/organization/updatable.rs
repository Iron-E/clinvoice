use
{
	std::fs,

	super::BincodeOrganization,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Updatable,
};

impl Updatable for BincodeOrganization<'_, '_>
{
	type Error = Error;

	fn update(&self) -> Result<()>
	{
		let serialized = bincode::serialize(&self.organization)?;
		fs::write(self.filepath(), serialized)?;
		Ok(())
	}
}
