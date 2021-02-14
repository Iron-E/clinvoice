mod deletable;
mod initializable;
mod into_organization_result;
mod job_adapter;
mod updatable;

use std::path::PathBuf;

clinvoice_adapter::Adapt!(Job => BincodeJob);

impl<'path> BincodeJob<'_, 'path, '_>
{
	/// # Summary
	///
	/// Return the directory within `store` that contains information about [`BincodeEmployee`]s.
	///
	/// # Parameters
	///
	/// * `store`, the [`Store`] whose `path` should be used to reference information about
	///   [`BincodeEmployee`]s.
	///
	/// # Returns
	///
	/// The [`Path`] leading to where [`BincodeEmployee`]s are in `store`.
	pub fn path(store: &Store<'_, 'path, '_>) -> PathBuf
	{
		return PathBuf::new().join(store.path).join("Jobs");
	}

	/// # Summary
	///
	/// Get the [`PathBuf`] pointing to where this [`BincodeJob`] is stored.
	///
	/// # Returns
	///
	/// A [`PathBuf`] pointing to where this [`BincodeJob`] is stored.
	pub fn filepath(&self) -> PathBuf
	{
		return Self::path(&self.store).join(self.job.id.to_string());
	}
}
