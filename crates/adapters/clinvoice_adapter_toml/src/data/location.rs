mod crud_location;
mod from_location;
mod display;

use clinvoice_data::Location;

/// # Summary
///
/// A wrapper around [`Location`] for use with TomlDB.
pub struct TomlLocation<'name>
(
	Location<'name>,
);
