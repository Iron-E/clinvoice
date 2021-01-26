mod deletable;
mod into_organization_result;
mod job_adapter;
mod updatable;

use std::path::PathBuf;

clinvoice_adapter::Adapt!(Job => TomlJob);

impl<'path> TomlJob<'_, '_, '_, '_, 'path, '_>
{
	/// # Summary
	///
	/// Return the directory within `store` that contains information about [`TomlEmployee`]s.
	///
	/// # Parameters
	///
	/// * `store`, the [`Store`] whose `path` should be used to reference information about
	///   [`TomlEmployee`]s.
	///
	/// # Returns
	///
	/// The [`Path`] leading to where [`TomlEmployee`]s are in `store`.
	pub fn path(store: &Store<'_, 'path, '_>) -> PathBuf
	{
		return PathBuf::new().join(store.path).join("Jobs");
	}
}
