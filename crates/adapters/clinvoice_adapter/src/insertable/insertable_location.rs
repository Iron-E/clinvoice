use crate::Wrapper;

pub trait InsertableLocation<T, W> where W : Wrapper<T>
{
	/// # Summary
	///
	/// Create a new `Location` with a generated ID.
	///
	/// # Parameters
	///
	/// * `name`, the name of the location.
	///
	/// # Returns
	///
	/// ```rust
	/// Location { name, id: /* generated */ };
	/// ```
	fn insert(name: &'_ str) -> W;

	/// # Summary
	///
	/// Create a new [`Location`] which is inside of `self`.
	///
	/// # Parameters
	///
	/// * `name`, the name of the inner location.
	///
	/// # Returns
	///
	/// ```rust
	/// Location { name, id: /* generated */, outside_id: self.unroll().id };
	/// ```
	fn insert_inner(&self, name: &'_ str) -> W;
}
