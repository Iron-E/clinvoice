use
{
	std::fs,

	super::{Config, Error, Result},

	clinvoice_adapter::data::Updatable,
};

impl Updatable for Config<'_, '_,>
{
	type Error = Error;

	fn update(&self) -> Result<()>
	{
		let path = Self::path();

		if let Some(parent) = path.parent()
		{
			if !parent.is_dir() { fs::create_dir_all(parent)?; }
		}

		let serialized = toml::to_string_pretty(self)?;
		fs::write(path, serialized)?;

		Ok(())
	}
}
