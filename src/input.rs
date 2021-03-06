mod error;
pub mod util;

pub use error::{Error, Result};

use
{
	core::
	{
		fmt::{Display, Debug},
		str::FromStr,
	},
	std::{any, io},

	clinvoice_adapter::data::Error as DataError,
	clinvoice_data::views::RestorableSerde,

	dialoguer::{Editor, Input, MultiSelect, Select},
	serde::{de::DeserializeOwned, Serialize},
	serde_yaml as yaml,
};

/// # Summary
///
/// Gather input from the user's text editor of choice.
///
/// # Remarks
///
/// The user's specified `$EDITOR` environment variable will be opened first, followed by whichever
/// editor is discovered by the [`edit_file`](edit::edit_file) function.
///
/// # Returns
///
/// * The deserialized entity with values filled in by the user.
/// * An [`Error`] encountered while creating, editing, or removing the temporary file.
pub fn edit<T>(entity: &T, prompt: impl AsRef<str>) -> Result<T> where
	T : DeserializeOwned + Serialize
{
	let serialized = yaml::to_string(&entity)?;
	let to_edit = format!("# {}\n\n{}", prompt.as_ref().replace('\n', "\n# "), serialized);

	let result = Editor::new().extension(".yaml").edit(&to_edit)?;
	let edited = result.ok_or(Error::NotEdited)?;
	yaml::from_str(&edited).map_err(|e| e.into())
}

/// # Summary
///
/// Gather input from the user's text editor of choice.
///
/// # Remarks
///
/// The user's specified `$EDITOR` environment variable will be opened first, followed by whichever
/// editor is discovered by the [`edit_file`](edit::edit_file) function.
///
/// # Returns
///
/// * The deserialized entity with values filled in by the user.
/// * An [`Error`] encountered while creating, editing, or removing the temporary file.
pub fn edit_and_restore<T>(entity: &T, prompt: impl AsRef<str>) -> Result<T> where
	T : DeserializeOwned + RestorableSerde + Serialize
{
	let mut edited = edit(entity, prompt)?;
	edited.restore(entity);
	Ok(edited)
}

/// # Summary
///
/// Gather input from the user's text editor of choice.
///
/// # Remarks
///
/// The user's specified `$EDITOR` environment variable will be opened first, followed by whichever
/// editor is discovered by the [`edit_file`](edit::edit_file) function.
///
/// # Returns
///
/// * The deserialized entity with values filled in by the user.
/// * An [`Error`] encountered while creating, editing, or removing the temporary file.
pub fn edit_default<T>(prompt: impl AsRef<str>) -> Result<T> where
	T : Default + DeserializeOwned + Serialize
{
	let default = T::default();
	match edit(&default, prompt)
	{
		Ok(d) => Ok(d),
		Err(Error::NotEdited) => Ok(default),
		Err(e) => Err(e),
	}
}

/// # Summary
///
/// [Edit](edit_func) markdown based on some `prompt` which will appear in the user's editor.
///
/// # Errors
///
/// * [`io::Error`] when the [edit][edit_func] fails.
/// * [`Error::NotEdited`] when the user does not change the `prompt`.
///
/// [edit_func]: Editor::edit
pub fn edit_markdown(prompt: &str) -> Result<String>
{
	let result = Editor::new().extension(".md").edit(prompt)?;
	result.ok_or(Error::NotEdited)
}

/// # Summary
///
/// `prompt` users to select elements from `entities`, then return them.
///
/// # Returns
///
/// * The selected entities.
/// * An [`Error`] incurred while selecting.
pub fn select<T>(entities: &[T], prompt: impl Into<String>) -> io::Result<Vec<T>> where
	T : Clone + Display
{
	if entities.is_empty() { return Ok(Vec::new()); }

	let selection = MultiSelect::new().items(entities).paged(true).with_prompt(prompt).interact()?;

	Ok(entities.iter().enumerate().filter_map(
		|(i, entity)| match selection.binary_search(&i)
		{
			Ok(_) => Some(entity.clone()),
			_ => None,
		},
	).collect())
}

/// # Summary
///
/// `prompt` users to select one element from `entities`, then return it.
///
/// # Returns
///
/// * The selected entity.
/// * An [`Error`] incurred while selecting.
pub fn select_one<T>(entities: &[T], prompt: impl Into<String>) -> Result<T> where
	T : Clone + Display
{
	if entities.is_empty()
	{
		return Err(DataError::NoData(format!("`{}`", any::type_name::<T>())).into());
	}

	let selector =
	{
		let mut s = Select::new();
		s.items(entities).paged(true).with_prompt(prompt);
		s
	};

	loop
	{
		return match selector.interact()
		{
			Ok(index) => Ok(entities[index].clone()),
			Err(e) if !(e.kind() == io::ErrorKind::Other && e.to_string().contains("Quit not allowed")) => Err(e.into()),
			_ =>
			{
				println!("Please select something, or press Ctrl+C to quit");
				continue
			},
		}
	}
}

/// # Summary
///
/// `prompt` the user to enter text.
pub fn text<S, T>(default_text: Option<T>, prompt: S) -> io::Result<T> where
	S : Into<String>,
	T : Clone + FromStr + Display,
	T::Err : Display + Debug,
{
	let mut input = Input::new();
	input.with_prompt(prompt);

	if let Some(text) = default_text { input.default(text); }

	input.interact_text()
}
