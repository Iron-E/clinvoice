use crate::Store;
use std::error::Error;

pub trait Initializable<E> where E : Error
{
	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store) -> Result<(), E>;
}
