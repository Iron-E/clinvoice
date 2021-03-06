use
{
	core::fmt::Display,

	super::menu,
	crate::{app::QUERY_PROMPT, DynResult, input},

	clinvoice_adapter::{data::PersonAdapter, Store},
	clinvoice_data::views::PersonView,
	clinvoice_query as query,
};

/// # Summary
///
/// Retrieve all [`Person`][person]s from the specified `store`. If no
/// [`Person`][person]s are retrieved, return an [error](DataError::NoData).
///
/// # Errors
///
/// * If the [retrieval][P_retrieve] operation fails, its error is forwarded.
/// * If no [`Person`][person]s are [retrieved][P_retrieve], an [`Error::NoData`] is returned.
/// * If the [selection](input::select) operation fails, its error is forwarded.
///
/// [P_retrieve]: clinvoice_adapter::data::PersonAdapter::retrieve
/// [person]: clinvoice_data::Person
pub fn retrieve_views<'err, D, P>(prompt: D, retry_on_empty: bool, store: &Store) -> DynResult<'err, Vec<PersonView>> where
	D : Display,
	P : PersonAdapter,

	<P as PersonAdapter>::Error : 'err,
{
	let query: query::Person = input::edit_default(format!("{}\n{}persons", prompt, QUERY_PROMPT))?;

	let results = P::retrieve(&query, &store)?;
	let results_view: Result<Vec<_>, _> = results.into_iter().map(PersonView::from).filter_map(|view| match query.matches_view(&view)
	{
		Ok(b) if b => Some(Ok(view)),
		Err(e) => Some(Err(e)),
		_ => None,
	}).collect();

	if retry_on_empty && results_view.as_ref().map(|r| r.is_empty()).unwrap_or(false) && menu::retry_query()?
	{
		return retrieve_views::<D, P>(prompt, true, store);
	}

	results_view.map_err(|e| e.into())
}
